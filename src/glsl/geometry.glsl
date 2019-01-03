#version 450 core

layout ( points ) in;
layout ( triangle_strip, max_vertices = 4 ) out;

in vec4 Color[];
out vec4 GeometryColor;
out vec2 RelPos;

uniform mat4 View;
uniform mat4 Projection;

void main(void){
  GeometryColor = Color[0];

  vec4 offsetPos = gl_in[0].gl_Position + vec4(-0.1, 0.1, 0.0, 0.0); 
  gl_Position = Projection * View * offsetPos;
  RelPos = vec2(-1.0, 1.0);
  EmitVertex();

  offsetPos = gl_in[0].gl_Position + vec4(-0.1, -0.1, 0.0, 0.0); 
  gl_Position = Projection * View * offsetPos;
  RelPos = vec2(-1.0, -1.0);
  EmitVertex();

  offsetPos = gl_in[0].gl_Position + vec4(0.1, 0.1, 0.0, 0.0); 
  gl_Position = Projection * View * offsetPos;
  RelPos = vec2(1.0, 1.0);
  EmitVertex();

  offsetPos = gl_in[0].gl_Position + vec4(0.1, -0.1, 0.0, 0.0); 
  gl_Position = Projection * View * offsetPos;
  RelPos = vec2(1.0, -1.0);
  EmitVertex();

  EndPrimitive();
}