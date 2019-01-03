#version 450 core

layout (location = 0) in vec3 VertexPosition;
layout (location = 1) in vec4 VertexColor;

out vec4 Color;

uniform mat4 Model;

void main()
{
    Color = VertexColor;
    gl_Position = Model * vec4(VertexPosition, 1.0);
}