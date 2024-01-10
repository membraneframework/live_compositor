use std::{collections::HashMap, time::Duration};

use log::error;

use crate::{
    state::renderers::Renderers, transformations::text_renderer::TextRendererCtx, InputId,
    OutputId, Resolution,
};

use super::{
    image_component::StatefulImageComponent,
    input_stream_component::StatefulInputStreamComponent,
    layout::{LayoutNode, SizedLayoutComponent, StatefulLayoutComponent},
    shader_component::StatefulShaderComponent,
    text_component::StatefulTextComponent,
    validation::validate_scene_update,
    web_view_component::StatefulWebViewComponent,
    ComponentId, Node, NodeParams, OutputScene, Position, SceneError, Size, StatefulComponent,
};

pub(super) struct BuildStateTreeCtx<'a> {
    pub(super) prev_state: HashMap<ComponentId, &'a StatefulComponent>,
    pub(super) last_render_pts: Duration,
    pub(super) renderers: &'a Renderers,
    pub(super) text_renderer_ctx: &'a TextRendererCtx,
    pub(super) input_resolutions: &'a HashMap<InputId, Resolution>,
}

pub(crate) struct SceneState {
    outputs: Vec<OutputSceneState>,
    last_pts: Duration,
    // Input resolutions from the last render
    input_resolutions: HashMap<InputId, Resolution>,
}

#[derive(Debug, Clone)]
struct OutputSceneState {
    output_id: OutputId,
    root: StatefulComponent,
    resolution: Resolution,
}

pub(crate) struct OutputNode {
    pub(crate) output_id: OutputId,
    pub(crate) node: Node,
    pub(crate) resolution: Resolution,
}

impl SceneState {
    pub fn new() -> Self {
        Self {
            outputs: vec![],
            last_pts: Duration::ZERO,
            input_resolutions: HashMap::new(),
        }
    }

    /// Function that should be called for each render. It's intended to keep state of
    /// SceneState in sync with layout code that is executed inside nodes.
    pub(crate) fn register_render_event(
        &mut self,
        pts: Duration,
        input_resolutions: HashMap<InputId, Resolution>,
    ) {
        self.last_pts = pts;
        self.input_resolutions = input_resolutions;
        // TODO: pass input stream sizes and populate it in the ComponentState tree
    }

    pub(crate) fn update_scene(
        &mut self,
        outputs: Vec<OutputScene>,
        renderers: &Renderers,
        text_renderer_ctx: &TextRendererCtx,
    ) -> Result<Vec<OutputNode>, SceneError> {
        validate_scene_update(&outputs)?;

        // Recalculate last known states from previous scene.
        //
        // This works because only scene update can modify a state,
        // but if we implement e.g. animations that trigger on size 
        // change or if input stream are missing then this code will
        // need to be executed per render.
        for output in self.outputs.iter_mut() {
            recalculate_layout(
                &mut output.root,
                Some(output.resolution.into()),
                self.last_pts,
                false,
            )
        }

        let ctx = BuildStateTreeCtx {
            prev_state: self
                .outputs
                .iter()
                .flat_map(|o| {
                    let mut components = HashMap::new();
                    gather_components_with_id(&o.root, &mut components);
                    components
                })
                .collect(),
            last_render_pts: self.last_pts,
            input_resolutions: &self.input_resolutions,
            text_renderer_ctx,
            renderers,
        };
        let output_states = outputs
            .into_iter()
            .map(|o| {
                Ok(OutputSceneState {
                    output_id: o.output_id,
                    root: o.root.stateful_component(&ctx)?,
                    resolution: o.resolution,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let nodes = output_states
            .iter()
            .map(|output| {
                Ok(OutputNode {
                    output_id: output.output_id.clone(),
                    node: output
                        .root
                        .intermediate_node()
                        .build_tree(Some(output.resolution), self.last_pts)?,
                    resolution: output.resolution,
                })
            })
            .collect::<Result<_, _>>()?;
        self.outputs = output_states;
        Ok(nodes)
    }
}

/// Intermediate representation of a node tree while it's being constructed.
pub(super) enum IntermediateNode {
    InputStream(StatefulInputStreamComponent),
    Shader {
        shader: StatefulShaderComponent,
        children: Vec<IntermediateNode>,
    },
    WebView {
        web: StatefulWebViewComponent,
        children: Vec<IntermediateNode>,
    },
    Image(StatefulImageComponent),
    Text(StatefulTextComponent),
    Layout {
        root: StatefulLayoutComponent,
        children: Vec<IntermediateNode>,
    },
}

impl IntermediateNode {
    /// * `resolution` - Defines desired resolution of the node. Each recursive call to build_tree
    ///   except the first one should call it with None.
    /// * `pts` - PTS from the last render (this function is not called on render
    ///   so we can't have exact PTS here)
    fn build_tree(self, resolution: Option<Resolution>, pts: Duration) -> Result<Node, SceneError> {
        match self {
            IntermediateNode::InputStream(input) => Ok(Node {
                params: NodeParams::InputStream(input.component.input_id),
                children: vec![],
            }),
            IntermediateNode::Shader { shader, children } => Ok(Node {
                params: NodeParams::Shader(shader.component, shader.shader),
                children: children
                    .into_iter()
                    .map(|node| node.build_tree(None, pts))
                    .collect::<Result<_, _>>()?,
            }),
            IntermediateNode::WebView { web, children } => Ok(Node {
                params: NodeParams::Web(web.instance),
                children: children
                    .into_iter()
                    .map(|node| node.build_tree(None, pts))
                    .collect::<Result<_, _>>()?,
            }),
            IntermediateNode::Layout { root, children } => {
                let size = match resolution {
                    Some(resolution) => resolution.into(),
                    None => Self::layout_node_size(pts, &root)?,
                };
                Ok(Node {
                    params: NodeParams::Layout(LayoutNode {
                        root: SizedLayoutComponent::new(root, size),
                    }),
                    children: children
                        .into_iter()
                        .map(|node| node.build_tree(None, pts))
                        .collect::<Result<_, _>>()?,
                })
            }
            IntermediateNode::Image(image) => Ok(Node {
                params: NodeParams::Image(image.image),
                children: vec![],
            }),
            IntermediateNode::Text(text) => Ok(Node {
                params: NodeParams::Text(text.params),
                children: vec![],
            }),
        }
    }

    fn layout_node_size(
        pts: Duration,
        layout: &StatefulLayoutComponent,
    ) -> Result<Size, SceneError> {
        let (width, height) = match layout.position(pts) {
            Position::Static { width, height } => (width, height),
            // Technically absolute positioning is a bug here, but I think throwing error
            // in this case would be to invasive. It's better to just ignore those values.
            Position::Absolute(position) => (Some(position.width), Some(position.height)),
        };
        if let (Some(width), Some(height)) = (width, height) {
            Ok(Size { width, height })
        } else {
            Err(SceneError::UnknownDimensionsForLayoutNodeRoot {
                component: layout.component_type(),
                msg: match layout.component_id() {
                    Some(id) => format!(
                        "Please provide width and height values for component with id \"{id}\""
                    ),
                    None => "Please provide width and height values.".to_string(),
                },
            })
        }
    }
}

fn recalculate_layout(
    component: &mut StatefulComponent,
    size: Option<Size>,
    pts: Duration,
    parent_is_layout: bool,
) {
    let (width, height) = (component.width(pts), component.height(pts));
    match component {
        StatefulComponent::Layout(layout) => {
            if !parent_is_layout {
                let size = size.or_else(|| {
                    let (Some(width), Some(height)) = (width, height) else {
                        error!("Unknown dimensions on root layout component.");
                        return None;
                    };
                    Some(Size { width, height })
                });
                if let Some(size) = size {
                    layout.layout(size, pts);
                }
            }
            for child in layout.children_mut() {
                recalculate_layout(child, None, pts, true)
            }
        }
        component => {
            for child in component.children_mut() {
                recalculate_layout(child, None, pts, false)
            }
        }
    }
}

fn gather_components_with_id<'a>(
    component: &'a StatefulComponent,
    components: &mut HashMap<ComponentId, &'a StatefulComponent>,
) {
    match component {
        StatefulComponent::InputStream(input) => {
            if let Some(id) = input.component_id() {
                components.insert(id.clone(), component);
            }
        }
        StatefulComponent::Shader(shader) => {
            if let Some(id) = shader.component_id() {
                components.insert(id.clone(), component);
            }
            for child in shader.children.iter() {
                gather_components_with_id(child, components);
            }
        }
        StatefulComponent::WebView(web) => {
            if let Some(id) = web.component_id() {
                components.insert(id.clone(), component);
            }
            for child in web.children.iter() {
                gather_components_with_id(child, components);
            }
        }
        StatefulComponent::Image(image) => {
            if let Some(id) = image.component_id() {
                components.insert(id.clone(), component);
            }
        }
        StatefulComponent::Text(image) => {
            if let Some(id) = image.component_id() {
                components.insert(id.clone(), component);
            }
        }
        StatefulComponent::Layout(layout) => {
            if let Some(id) = layout.component_id() {
                components.insert(id.clone(), component);
            }
            for child in layout.children() {
                gather_components_with_id(child, components);
            }
        }
    }
}
