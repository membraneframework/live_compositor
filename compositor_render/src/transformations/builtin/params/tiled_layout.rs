use compositor_common::{
    scene::{builtin_transformations::tailed_layout::TailedLayoutSpec, Resolution},
    util::align::{HorizontalAlign, VerticalAlign},
};

use nalgebra_glm::Mat4;

use crate::transformations::builtin::box_layout::BoxLayout;

#[derive(Debug)]
struct RowsCols {
    rows: u32,
    cols: u32,
}

impl RowsCols {
    pub fn from_rows_count(inputs_count: u32, rows: u32) -> Self {
        let cols = ceil_div(inputs_count, rows);
        Self { rows, cols }
    }
}

#[derive(Debug)]
pub struct TiledLayoutParams {
    transformation_matrices: Vec<Mat4>,
}

impl TiledLayoutParams {
    pub fn new(input_resolutions: &[Option<Resolution>], spec: &TailedLayoutSpec) -> Self {
        let inputs = input_resolutions
            .iter()
            .filter_map(|input_resolution| *input_resolution);

        let inputs_count = inputs.clone().count() as u32;

        // This should fallback anyway
        if inputs_count == 0 {
            return Self {
                transformation_matrices: vec![Mat4::identity()],
            };
        }

        let optimal_rows_cols = Self::optimize_inputs_layout(inputs_count, spec);

        let tile_size = Self::tile_size(&optimal_rows_cols, spec);

        let tiles_layout = Self::layout_tiles(inputs_count, &optimal_rows_cols, tile_size, spec);

        let transformation_matrices: Vec<Mat4> = tiles_layout
            .iter()
            .zip(inputs)
            .map(|(tile_layout, input_resolution)| {
                Self::transformation_matrix(tile_layout, input_resolution, spec.resolution)
            })
            .collect();

        Self {
            transformation_matrices,
        }
    }

    fn layout_tiles(
        inputs_count: u32,
        rows_cols: &RowsCols,
        tile_size: Resolution,
        spec: &TailedLayoutSpec,
    ) -> Vec<BoxLayout> {
        let mut layouts = Vec::with_capacity(inputs_count as usize);

        // Because scaled tails with padding and margin don't have to cover whole output frame,
        // additional padding is distributed is distributed accordingly to alignment
        let additional_y_padding = spec.resolution.height as u32
            - (tile_size.height as u32 + 2 * spec.padding) * rows_cols.rows
            - (spec.margin * (rows_cols.rows + 1));

        let (additional_top_padding, justified_padding_y) = match spec.vertical_alignment {
            VerticalAlign::Top => (0.0, 0.0),
            VerticalAlign::Center => (additional_y_padding as f32 / 2.0, 0.0),
            VerticalAlign::Bottom => (additional_y_padding as f32, 0.0),
            VerticalAlign::Justified => {
                let space = additional_y_padding as f32 / (rows_cols.rows + 1) as f32;
                (0.0, space)
            }
        };

        let mut top =
            additional_top_padding + justified_padding_y + spec.padding as f32 + spec.margin as f32;
        for row in 0..rows_cols.rows {
            let tiles_in_row = if row < rows_cols.rows - 1 {
                rows_cols.cols
            } else {
                inputs_count - ((rows_cols.rows - 1) * rows_cols.cols)
            };

            let additional_x_padding = spec.resolution.width as u32
                - (tile_size.width as u32 + 2 * spec.padding) * tiles_in_row
                - (spec.margin * (tiles_in_row + 1));

            let (additional_left_padding, justified_padding_x) = match spec.horizontal_alignment {
                HorizontalAlign::Left => (0.0, 0.0),
                HorizontalAlign::Right => (additional_x_padding as f32, 0.0),
                HorizontalAlign::Justified => {
                    let space = additional_x_padding as f32 / (tiles_in_row + 1) as f32;
                    (0.0, space)
                }
                HorizontalAlign::Center => (additional_x_padding as f32 / 2.0, 0.0),
            };

            let mut left = additional_left_padding
                + justified_padding_x
                + spec.margin as f32
                + spec.padding as f32;

            for _col in 0..tiles_in_row {
                layouts.push(BoxLayout {
                    top_left_corner: (left, top),
                    width: tile_size.width as f32,
                    height: tile_size.height as f32,
                    rotation_degrees: 0.0,
                });

                left += tile_size.width as f32
                    + spec.margin as f32
                    + spec.padding as f32 * 2.0
                    + justified_padding_x;
            }
            top += tile_size.height as f32
                + spec.margin as f32
                + spec.padding as f32 * 2.0
                + justified_padding_y;
        }

        layouts
    }

    fn tile_size(rows_cols: &RowsCols, spec: &TailedLayoutSpec) -> Resolution {
        let x_padding = (rows_cols.cols * 2 * spec.padding) as f32;
        let y_padding = (rows_cols.rows * 2 * spec.padding) as f32;
        let x_margin = ((rows_cols.cols + 1) * spec.margin) as f32;
        let y_margin = ((rows_cols.rows + 1) * spec.margin) as f32;

        let x_scale = (spec.resolution.width as f32 - x_padding - x_margin).max(0.0)
            / rows_cols.cols as f32
            / spec.tile_aspect_ratio.0 as f32;
        let y_scale = (spec.resolution.height as f32 - y_padding - y_margin).max(0.0)
            / rows_cols.rows as f32
            / spec.tile_aspect_ratio.1 as f32;

        let scale = if x_scale < y_scale { x_scale } else { y_scale };

        Resolution {
            width: (spec.tile_aspect_ratio.0 as f32 * scale) as usize,
            height: (spec.tile_aspect_ratio.1 as f32 * scale) as usize,
        }
    }

    pub fn shader_buffer_content(&self) -> bytes::Bytes {
        let mut matrices_bytes = bytes::BytesMut::new();
        for matrix in &self.transformation_matrices {
            let colum_based = matrix.transpose();
            for el in &colum_based {
                matrices_bytes.extend_from_slice(&el.to_ne_bytes())
            }
        }

        matrices_bytes.freeze()
    }

    /// Optimize number of rows and cols to maximize space covered by tiles,
    /// preserving tile aspect_ratio
    fn optimize_inputs_layout(inputs_count: u32, spec: &TailedLayoutSpec) -> RowsCols {
        let mut best_rows_cols = RowsCols::from_rows_count(inputs_count, 1);
        let mut best_tile_width = 0;

        for rows in 1..=inputs_count {
            let rows_cols = RowsCols::from_rows_count(inputs_count, rows);
            // larger width <=> larger tile size, because of const tile aspect ratio
            let tile_size = Self::tile_size(&rows_cols, spec).width;

            if tile_size > best_tile_width {
                best_rows_cols = rows_cols;
                best_tile_width = tile_size;
            }
        }

        best_rows_cols
    }

    fn transformation_matrix(
        tile_layout: &BoxLayout,
        input_resolution: Resolution,
        output_resolution: Resolution,
    ) -> Mat4 {
        tile_layout
            .fit(
                input_resolution,
                HorizontalAlign::Center,
                VerticalAlign::Center,
            )
            .transformation_matrix(output_resolution)
    }
}

fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}