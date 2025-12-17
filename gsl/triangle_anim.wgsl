struct Uniforms {
    position: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5),
    );
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );
    
    var pos = positions[idx];
    
    // Wind effect parameters
    let wind_strength = 0.1;
    let wind_speed = 2.0;
    let wave_frequency = 3.0;
    
    // Apply horizontal wind wave (more at the top)
    let height_factor = (pos.y + 0.5); // 0.0 at bottom, 1.0 at top
    let wind_offset = sin(uniforms.time * wind_speed + pos.y * wave_frequency) * wind_strength * height_factor;
    pos.x += wind_offset;
    
    // Add slight vertical sway
    let sway = sin(uniforms.time * wind_speed * 0.5) * 0.03 * height_factor;
    pos.y += sway;
    
    // Apply user position
    pos += uniforms.position;
    
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.color = colors[idx];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}