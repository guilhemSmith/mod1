#version 400 core
out vec4 FragColor;

in vec3 fragPos;

uniform sampler2D screenTexture;
uniform vec3 viewPos;

void main()
{
   FragColor = vec4(vec3(0.1, 0.5, 0.6), 1.0);
}