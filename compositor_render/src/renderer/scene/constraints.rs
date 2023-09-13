use compositor_common::{
    error::ConstraintsValidationError,
    scene::{
        validation::{constraints::NodeConstraints, inputs::InputsCountConstraint},
        NodeParams, NodeSpec, SceneSpec,
    },
};

pub fn validate_constraints(scene: &SceneSpec) -> Result<(), ConstraintsValidationError> {
    for (node_id, node_constraints) in scene
        .nodes
        .iter()
        .map(|node| (&node.node_id, node_constraints(node)))
    {
        node_constraints.validate(scene, node_id)?
    }

    Ok(())
}

fn node_constraints(node: &NodeSpec) -> NodeConstraints {
    // TODO: make web renderer and shader constraints API configurable
    match &node.params {
        NodeParams::WebRenderer { .. } => NodeConstraints {
            inputs_count: InputsCountConstraint::Bounded {
                minimal: 0,
                maximal: 16,
            },
        },
        NodeParams::Shader { .. } => NodeConstraints {
            inputs_count: InputsCountConstraint::Bounded {
                minimal: 0,
                maximal: 16,
            },
        },
        NodeParams::TextRenderer { .. } => NodeConstraints {
            inputs_count: InputsCountConstraint::Exact(0),
        },
        NodeParams::Image { .. } => NodeConstraints {
            inputs_count: InputsCountConstraint::Exact(0),
        },
        NodeParams::Builtin { transformation } => NodeConstraints {
            inputs_count: transformation.inputs_constrains(),
        },
    }
}
