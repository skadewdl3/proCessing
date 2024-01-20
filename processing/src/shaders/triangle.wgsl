struct Uniforms {
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32,
  x3: f32,
  y3: f32,
};

struct VertexInput {
  @location(0) position: vec3<f32>,
  // @location(1) color: vec4<f32>
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  // @location(0) color: vec4<f32>
}

// struct VOutput {
//   @builtin(position) position: vec4<f32>,
// };

// @group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main (vertex_data: VertexInput) -> VertexOutput {

  var output: VertexOutput;
  output.position = vec4<f32>(vertex_data.position, 1.0);
  // output.color = vertex_data.color;

  return output;
}


@fragment
fn fs_main (fragment_data: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}