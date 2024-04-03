use std::{fmt::Display, path::PathBuf, sync::Arc, time::Duration};

use super::utils::{create_renderer, frame_to_rgba, snaphot_save_path, snapshots_diff};

use anyhow::Result;
use compositor_render::{
    scene::RGBColor, Frame, FrameSet, InputId, OutputId, Renderer, Resolution, YuvData,
};
use image::ImageBuffer;
use video_compositor::types;

pub(super) const OUTPUT_ID: &str = "output_id";

pub struct TestCase {
    pub name: &'static str,
    pub inputs: Vec<TestInput>,
    pub renderers: Vec<&'static str>,
    pub timestamps: Vec<Duration>,
    pub scene_updates: Updates,
    pub only: bool,
    pub allowed_error: f32,
}

pub enum Updates {
    Scene(&'static str, Resolution),
    Scenes(Vec<(&'static str, Resolution)>),
}

impl Default for TestCase {
    fn default() -> Self {
        Self {
            name: "",
            inputs: Vec::new(),
            renderers: Vec::new(),
            timestamps: vec![Duration::from_secs(0)],
            scene_updates: Updates::Scenes(vec![]),
            only: false,
            allowed_error: 20.0,
        }
    }
}

pub struct TestCaseInstance {
    pub case: TestCase,
    pub renderer: Renderer,
}

impl TestCaseInstance {
    pub fn new(test_case: TestCase) -> TestCaseInstance {
        if test_case.name.is_empty() {
            panic!("Snapshot test name has to be provided");
        }

        let mut renderer = create_renderer();
        // TODO: fix register requests
        // for json in test_case.renderers.iter() {
        //     let spec = register_requests_to_renderers(
        //         serde_json::from_str::<RegisterRequest>(json).unwrap(),
        //     );
        //     if matches!(spec, RendererSpec::WebRenderer(_)) {
        //         panic!("Tests with web renderer are not supported");
        //     }
        //     renderer.register_renderer(spec).unwrap();
        // }

        for (index, _) in test_case.inputs.iter().enumerate() {
            renderer.register_input(InputId(format!("input_{}", index + 1).into()))
        }

        let outputs = match test_case.scene_updates {
            Updates::Scene(scene, resolution) => vec![(scene, resolution)],
            Updates::Scenes(ref scenes) => scenes.clone(),
        };

        for (update_str, resolution) in outputs {
            let scene: types::UpdateOutputRequest = serde_json::from_str(update_str).unwrap();
            if let Some(root) = scene.video {
                renderer
                    .update_scene(
                        OutputId(OUTPUT_ID.into()),
                        resolution,
                        root.try_into().unwrap(),
                    )
                    .unwrap();
            }
        }

        TestCaseInstance {
            case: test_case,
            renderer,
        }
    }

    #[allow(dead_code)]
    pub fn run(&self) -> Result<(), TestCaseError> {
        for pts in self.case.timestamps.iter() {
            let (_, test_result) = self.test_snapshots_for_pts(*pts);
            test_result?;
        }
        Ok(())
    }

    pub fn test_snapshots_for_pts(&self, pts: Duration) -> (Snapshot, Result<(), TestCaseError>) {
        let snapshot = self.snapshot_for_pts(pts).unwrap();

        let save_path = snapshot.save_path();
        if !save_path.exists() {
            return (
                snapshot.clone(),
                Err(TestCaseError::SnapshotNotFound(snapshot.clone())),
            );
        }

        let snapshot_from_disk = image::open(&save_path).unwrap().to_rgba8();
        let snapshots_diff = snapshots_diff(&snapshot_from_disk, &snapshot.data);
        if snapshots_diff > self.case.allowed_error {
            return (
                snapshot.clone(),
                Err(TestCaseError::Mismatch {
                    snapshot_from_disk: snapshot_from_disk.into(),
                    produced_snapshot: snapshot.clone(),
                    diff: snapshots_diff,
                }),
            );
        }

        if snapshots_diff > 0.0 {
            println!(
                "Snapshot error in range (allowed: {}, current: {})",
                self.case.allowed_error, snapshots_diff
            );
        }

        (snapshot, Ok(()))
    }

    #[allow(dead_code)]
    pub fn snapshot_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for pts in self.case.timestamps.iter() {
            paths.push(snaphot_save_path(self.case.name, pts));
        }

        paths
    }

    pub fn snapshot_for_pts(&self, pts: Duration) -> Result<Snapshot> {
        let mut frame_set = FrameSet::new(pts);
        for input in self.case.inputs.iter() {
            let input_id = InputId::from(Arc::from(input.name.clone()));
            let frame = Frame {
                data: input.data.clone(),
                resolution: input.resolution,
                pts,
            };
            frame_set.frames.insert(input_id, frame);
        }

        let outputs = self.renderer.render(frame_set)?;

        let output_frame = outputs.frames.get(&OutputId(OUTPUT_ID.into())).unwrap();
        let new_snapshot = frame_to_rgba(output_frame);
        Ok(Snapshot {
            test_name: self.case.name.to_owned(),
            pts,
            resolution: output_frame.resolution,
            data: new_snapshot,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TestInput {
    pub name: String,
    pub resolution: Resolution,
    pub data: YuvData,
}

impl TestInput {
    const COLOR_VARIANTS: [RGBColor; 17] = [
        // RED, input_0
        RGBColor(255, 0, 0),
        // GREEN, input_1
        RGBColor(0, 255, 0),
        // YELLOW, input_2
        RGBColor(255, 255, 0),
        // MAGENTA, input_3
        RGBColor(255, 0, 255),
        // BLUE, input_4
        RGBColor(0, 0, 255),
        // CYAN, input_5
        RGBColor(0, 255, 255),
        // ORANGE, input_6
        RGBColor(255, 165, 0),
        // WHITE, input_7
        RGBColor(255, 255, 255),
        // GRAY, input_8
        RGBColor(128, 128, 128),
        // LIGHT_RED, input_9
        RGBColor(255, 128, 128),
        // LIGHT_BLUE, input_10
        RGBColor(128, 128, 255),
        // LIGHT_GREEN, input_11
        RGBColor(128, 255, 128),
        // PINK, input_12
        RGBColor(255, 192, 203),
        // PURPLE, input_13
        RGBColor(128, 0, 128),
        // BROWN, input_14
        RGBColor(165, 42, 42),
        // YELLOW_GREEN, input_15
        RGBColor(154, 205, 50),
        // LIGHT_YELLOW, input_16
        RGBColor(255, 255, 224),
    ];

    pub fn new(index: usize) -> Self {
        Self::new_with_resolution(
            index,
            Resolution {
                width: 640,
                height: 360,
            },
        )
    }

    pub fn new_with_resolution(index: usize, resolution: Resolution) -> Self {
        let color = Self::COLOR_VARIANTS[index].to_yuv();
        let mut y_plane = vec![0; resolution.width * resolution.height];
        let mut u_plane = vec![0; (resolution.width * resolution.height) / 4];
        let mut v_plane = vec![0; (resolution.width * resolution.height) / 4];

        let yuv_color = |x: usize, y: usize| {
            const BORDER_SIZE: usize = 18;
            const GRID_SIZE: usize = 72;

            let is_border_in_x =
                x <= BORDER_SIZE || (x <= resolution.width && x >= resolution.width - BORDER_SIZE);
            let is_border_in_y: bool = y <= BORDER_SIZE
                || (y <= resolution.height && y >= resolution.height - BORDER_SIZE);
            let is_on_grid = (x / GRID_SIZE + y / GRID_SIZE) % 2 == 0;

            let mut y = color.0;
            if is_border_in_x || is_border_in_y || is_on_grid {
                y -= 0.2;
            }

            (y.clamp(0.0, 1.0), color.1, color.2)
        };

        for x_coord in 0..resolution.width {
            for y_coord in 0..resolution.height {
                let (y, u, v) = yuv_color(x_coord, y_coord);
                if x_coord % 2 == 0 && y_coord % 2 == 0 {
                    let (_, u2, v2) = yuv_color(x_coord + 1, y_coord);
                    let (_, u3, v3) = yuv_color(x_coord, y_coord + 1);
                    let (_, u4, v4) = yuv_color(x_coord + 1, y_coord + 1);

                    let coord = (y_coord / 2) * (resolution.width / 2) + (x_coord / 2);
                    u_plane[coord] = ((u + u2 + u3 + u4) * 64.0) as u8;
                    v_plane[coord] = ((v + v2 + v3 + v4) * 64.0) as u8;
                }

                y_plane[y_coord * resolution.width + x_coord] = (y * 255.0) as u8;
            }
        }

        let data = YuvData {
            y_plane: y_plane.into(),
            u_plane: u_plane.into(),
            v_plane: v_plane.into(),
        };

        Self {
            name: format!("input_{index}"),
            resolution,
            data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub test_name: String,
    pub pts: Duration,
    pub resolution: Resolution,
    pub data: Vec<u8>,
}

impl Snapshot {
    pub fn save_path(&self) -> PathBuf {
        snaphot_save_path(&self.test_name, &self.pts)
    }
}

#[derive(Debug)]
pub enum TestCaseError {
    SnapshotNotFound(Snapshot),
    Mismatch {
        snapshot_from_disk: Box<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
        produced_snapshot: Snapshot,
        diff: f32,
    },
}

impl std::error::Error for TestCaseError {}

impl Display for TestCaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            TestCaseError::SnapshotNotFound(Snapshot { test_name, pts, .. }) => format!(
                "FAILED: \"{}\", PTS({}). Snapshot file not found. Generate snapshots first",
                test_name,
                pts.as_secs()
            ),
            TestCaseError::Mismatch {
                produced_snapshot: Snapshot { test_name, pts, .. },
                diff,
                ..
            } => {
                format!(
                    "FAILED: \"{}\", PTS({}). Snapshots are different error={}",
                    test_name,
                    pts.as_secs_f32(),
                    diff,
                )
            }
        };

        f.write_str(&err_msg)
    }
}
