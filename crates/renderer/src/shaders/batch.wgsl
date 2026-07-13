struct Globals {
    view_projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
    @location(1) color: vec4<f32>,
};

// slot 0: per-vertex quad
@vertex
fn vs_main(
    @location(0) quad_pos: vec2<f32>,
    @location(1) quad_uv: vec2<f32>,
    // slot 1: per-instance
    @location(2) sprite_pos: vec2<f32>,
    @location(3) sprite_size: vec2<f32>,
    @location(4) sprite_uv: vec4<f32>,
    @location(5) sprite_color: vec4<f32>,
    @location(6) sprite_rotation: f32,
) -> VertexOutput {
    let c = cos(sprite_rotation);
    let s = sin(sprite_rotation);
    let rotated = vec2<f32>(
        quad_pos.x * c - quad_pos.y * s,
        quad_pos.x * s + quad_pos.y * c
    );
    let world_pos = rotated * sprite_size * 0.5 + sprite_pos;

    var out: VertexOutput;
    out.position = globals.view_projection * vec4<f32>(world_pos, 0.0, 1.0);
    out.texcoord = mix(sprite_uv.xy, sprite_uv.zw, quad_uv);
    out.color = sprite_color;
    return out;
}

@group(1) @binding(0)
var t_texture: texture_2d<f32>;
@group(1) @binding(1)
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.texcoord) * in.color;
}
