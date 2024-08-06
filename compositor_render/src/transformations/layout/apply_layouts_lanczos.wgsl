struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}


struct Layout {
    vertices_transformation: mat4x4<f32>,
    texture_coord_transformation: mat4x4<f32>,
    color: vec4<f32>, // used only when is_texture == 0
    is_texture: u32, // 0 -> color, 1 -> texture
    layout_resolution: vec2<f32>,
}

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(0) var<uniform> layouts: array<Layout, 128>;
@group(2) @binding(0) var sampler_: sampler;

var<push_constant> layout_id: u32;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    let vertices_transformation_matrix: mat4x4<f32> = layouts[layout_id].vertices_transformation;
    let texture_coord_transformation_matrix: mat4x4<f32> = layouts[layout_id].texture_coord_transformation;

    output.position = vec4(input.position, 1.0) * vertices_transformation_matrix;
    output.tex_coords = (vec4(input.tex_coords, 0.0, 1.0) * texture_coord_transformation_matrix).xy;

    return output;
}

const PI: f32 = 3.1415926535897932384626433832795;
// Lanczos parameter a
const A: f32 = 3.0;


// Lanczos Sinc Function
fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        return 1.0;
    }
    let x_pi = PI * x;
    return (sin(x_pi) / x_pi);
}

// Lanczos Weight Function
fn lanczos(x: f32) -> f32 {
    if abs(x) < A {
        return sinc(x) * sinc(x / A);
    } else {
        return 0.0;
    }
}


@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let current_layout = layouts[layout_id];
    let dim = textureDimensions(texture);
    
    let input_resolution: vec2<f32> = vec2<f32>(f32(dim.x), f32(dim.y));
    let scaling_factor: vec2<f32> = current_layout.layout_resolution / input_resolution;

    let half_pixel_size: vec2<f32> = vec2<f32>(0.5) / input_resolution;

    var color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var normalization: f32 = 0.0;


    for (var i: f32 = -A; i <= A; i += 1.0) {
        for (var j: f32 = -A; j <= A; j += 1.0) {
            // Position on input texture in pixels
            let x = input.tex_coords * input_resolution;
            // Sample position in pixels
            let sample_pixel_x = floor(input.tex_coords.x * input_resolution.x) + i;
            let sample_pixel_y = floor(input.tex_coords.y * input_resolution.y) + j;

            // Sample position in texture coordinates
            let sample_coord_x = clamp((sample_pixel_x / input_resolution.x) + half_pixel_size.x, 0.0, 1.0);
            let sample_coord_y = clamp((sample_pixel_y / input_resolution.y) + half_pixel_size.y, 0.0, 1.0);
            let sample_coords = vec2<f32>(sample_coord_x, sample_coord_y);
            
            // Distance of sampled output pixel from sampled position 
            let dx = sample_pixel_x - x.x;
            let dy = sample_pixel_y - x.y;

            let sample_color: vec4<f32> = textureSample(texture, sampler_, sample_coords);
            // Lanczos weight normalizing sampled values
            let weight: f32 = lanczos(dx) * lanczos(dy);
            color += sample_color * weight;
            normalization += weight;
        }
    }

    // sampling can't be conditional, so in case of plane_id == -1
    // sample textures[0], but ignore the result.
    if (current_layout.is_texture == 0u) {
        return current_layout.color;
    }
    // clamp transparent, when crop > input texture
    let is_inside: f32 = round(f32(input.tex_coords.x < 1.0 && input.tex_coords.x > 0.0 && input.tex_coords.y > 0.0 && input.tex_coords.y < 1.0));
    
    return is_inside * color / normalization;
}
