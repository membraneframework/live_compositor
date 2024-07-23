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
    layout_resolution: vec2<f32>,
    is_texture: u32, // 0 -> color, 1 -> texture
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
const A_f: f32 = 3.0;
const A: i32 = 3;


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
    if abs(x) < A_f {
        return sinc(x) * sinc(x / A_f);
    } else {
        return 0.0;
    }
}


@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let current_layout = layouts[layout_id];
    let dim = textureDimensions(texture);
    
    let input_resolution: vec2<f32> = vec2<f32>(f32(dim.x), f32(dim.y));
    let coord: vec2<f32> = input.tex_coords - (vec2<f32>(0.5) / input_resolution);
    
    let ccoord: vec2<f32> = floor(coord * input_resolution) / input_resolution;
    var color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var i: i32 = -A; i <= A; i++) {
        for (var j: i32 = -A; j <= A; j++) {
            let offs = vec2<f32>(f32(i), f32(j));

            let sco: vec2<f32> = (offs / input_resolution) + ccoord;
            let d = clamp((sco - coord) * input_resolution, vec2<f32>(-A_f), vec2<f32>(A_f));

            let sample_color: vec4<f32> = textureSample(texture, sampler_, ccoord);
            let weight: f32 = lanczos(d.x) * lanczos(d.y);
            color += sample_color * weight;
        }
    }

    // sampling can't be conditional, so in case of plane_id == -1
    // sample textures[0], but ignore the result.
    if (current_layout.is_texture == 0u) {
        return current_layout.color;
    }
    // clamp transparent, when crop > input texture
    let is_inside: f32 = round(f32(input.tex_coords.x < 1.0 && input.tex_coords.x > 0.0 && input.tex_coords.y > 0.0 && input.tex_coords.y < 1.0));
    
    return is_inside * color;
}
