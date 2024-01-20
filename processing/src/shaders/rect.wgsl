struct Uniforms {
  @location(0) stroke: vec4<f32>,
  @location(1) fill: vec4<f32>
}

struct VertexInput {
  @location(0) position: vec3<f32>,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
}

// @group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main (vertex_data: VertexInput) -> @builtin(position) vec4<f32> {

  var output: VertexOutput;
  return vec4<f32>(vertex_data.position, 1.0);
}


@fragment
fn fs_main () -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}