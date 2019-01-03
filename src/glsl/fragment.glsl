#version 450 core

in vec2 RelPos;
in vec4 GeometryColor;
out vec4 FragmentColor;

void main()
{
    float R = 1.0; 
    float dist = sqrt(dot(RelPos, RelPos));
    if (dist >= R)
    { discard; }
    FragmentColor = GeometryColor;
}