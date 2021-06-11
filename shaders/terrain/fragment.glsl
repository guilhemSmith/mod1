#version 400 core
out vec4 FragColor;

in vec3 color;
in vec3 fragPos;
in vec3 normal;

uniform vec3 viewPos;
uniform vec3 lightPos;

vec3 light_color(vec3 base_color, vec3 normal, vec3 fragPos, vec3 viewPos, vec3 lightPos);

void main()
{
   vec3 result = light_color(color, normal, fragPos, viewPos, lightPos);

   FragColor = vec4(result, 1.0);
}