struct CameraUniform {
  view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
  var x = f32(i32(in_vertex_index) - 1) * 0.5;
  var y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5; // [0, 1] -> [-0.5, 0.5]

  return camera.view_proj * vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
