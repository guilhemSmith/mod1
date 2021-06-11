#version 400 core
out vec4 FragColor;

in vec3 fragPos;

uniform sampler2D screenTexture;
uniform vec3 viewPos;

void main()
{
   FragColor = vec4(vec3(0.7, 0.7, 0.7), 1.0);
}