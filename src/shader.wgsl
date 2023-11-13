// VERTEX DATA
//0 = position
//1 = tex_coord0
//2 = normal
//3 = color
//4 =
//5 =

// CAMERA MATRIX
//6 = instance matrix line 0
//7 = instance matrix line 1
//8 = instance matrix line 2
//9 = instance matrix line 3

//TIME
//

struct CameraUniform{
    view_projection: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord0: vec4<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord0: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
            instance.model_matrix_0,
            instance.model_matrix_1,
            instance.model_matrix_2,
            instance.model_matrix_3,
        );

    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = camera.view_projection * model_matrix * vec4<f32>(model.position, 1.0);
    out.tex_coord0 = model.tex_coord0;
    return out;
}

// custom data aka textures
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var texture = textureSample(t_diffuse, s_diffuse, in.tex_coord0);
    var color = vec4<f32>(in.color,1.0);

    var output = texture + color * 0.5;
    return output;
}