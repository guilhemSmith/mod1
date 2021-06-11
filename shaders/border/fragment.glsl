#version 400 core
out vec4 FragColor;

in vec3 color;
in vec3 fragPos;
in vec3 normal;

uniform vec3 viewPos;
uniform vec3 lightPos;

vec3 light_color(vec3 base_color, vec3 normal, vec3 fragPos, vec3 viewPos, vec3 lightPos, float specularStrength, int shininess);

void main()
{
   if (fragPos.y > -15.0) {
      vec3 result = light_color(color, normal, fragPos, viewPos, lightPos, 0.25, 8) * (1.0 + fragPos.y / 15.0) + vec3(0.0) * (-fragPos.y) / 15.0;
      FragColor = vec4(result, 1.0);
   }
   else {
      FragColor = vec4(vec3(0.0), 1.0);
   }
}