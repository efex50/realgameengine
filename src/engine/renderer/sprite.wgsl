struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) scale: vec2<f32>,
    @location(4) rotation: f32,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let cos_r = cos(instance.rotation);
    let sin_r = sin(instance.rotation);

    // Apply scale then rotation
    let scaled_x = model.position.x * instance.scale.x;
    let scaled_y = model.position.y * instance.scale.y;

    let rot_x = scaled_x * cos_r - scaled_y * sin_r;
    let rot_y = scaled_x * sin_r + scaled_y * cos_r;

    // Apply translation
    let final_pos = vec2<f32>(rot_x + instance.position.x, rot_y + instance.position.y);

    var out: VertexOutput;
    // Map from world/screen to normalized device coordinates. 
    // Usually requires an orthographic camera matrix, but for this simple demo, 
    // passing the coordinates directly assuming NDC or dealing with it in CPU.
    out.clip_position = vec4<f32>(final_pos, 0.0, 1.0);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
