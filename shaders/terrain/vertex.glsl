#version 400 core
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
		if (aPos.y <= 15.0) {
			color = vec3(0.3, 0.8, 0.1) * aPos.y / 15.0 + vec3(0.8, 0.8, 0.3) * (1.0 - aPos.y / 15.0);
		}
		else {
			color = vec3(0.5, 0.3, 0.1) * (aPos.y - 15.0) / 35.0 + vec3(0.3, 0.8, 0.1) * (1.0 - (aPos.y - 15.0) / 35.0);
		}
	}
	else {
		if (aPos.y >= -15.0) {
			color = vec3(0.5, 0.5, 0.5) * (-aPos.y) / 15.0 + vec3(0.8, 0.8, 0.3) * (1.0 + aPos.y / 15.0);
		}
		else {
			color = vec3(0.1, 0.1, 0.1) * (-aPos.y - 15.0) / 35.0 + vec3(0.5, 0.5, 0.5) * (1.0 + (aPos.y + 15.0) / 35.0);
		}
	}
	fragPos = vec3(model * vec4(aPos, 1.0));
	normal = aNormal;
}