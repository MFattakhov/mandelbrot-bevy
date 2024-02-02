#import bevy_pbr::forward_io::VertexOutput

#import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d

struct Material {
    ul_re: f32,
    ul_im: f32,
    lr_re: f32,
    lr_im: f32,
};

@group(1) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(
    mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let c = vec2(mix(material.ul_re, material.lr_re, mesh.uv.x), mix(material.ul_im, material.lr_im, mesh.uv.y));
    var z = vec2(0.0, 0.0);

    let max_iterations = 127;
    for (var i = 1; i <= max_iterations; i++) {
        z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;

        if (z.x * z.x + z.y * z.y > 4.0) {
            let t = 1.0 - f32(i) / f32(max_iterations);
            return vec4(t, t, t, 1.0);
        }
    }

    return vec4(0.0, 0.0, 0.0, 1.0);
}

