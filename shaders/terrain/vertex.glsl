#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 color;
out vec3 fragPos;
out vec3 normal;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0f);
	if (aPos.y >= 0.0) {
		color = vec3(0.3, 0.8, 0.1) * aPos.y / 30.0 + vec3(0.8, 0.8, 0.3) * (1.0 - aPos.y / 30.0);
	}
	else {
		color = vec3(0.0, 0.0, 0.0) * (-aPos.y) / 10.0 + vec3(0.8, 0.8, 0.3) * (1.0 + aPos.y / 10.0);
	}
	fragPos = vec3(model * vec4(aPos, 1.0));
	normal = aNormal;
}