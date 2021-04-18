#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 color;
void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
	color = vec3(0.0, 0.8, 0.0);
}