#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> empty_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> progress: f32;
@group(1) @binding(2)
var<uniform> fill_color: vec4<f32>;

@fragment
fn fragment(
    mesh: UiVertexOutput,
) -> @location(0) vec4<f32> {
    if progress < mesh.uv.x {
        return empty_color;
    }
    return fill_color;
}
