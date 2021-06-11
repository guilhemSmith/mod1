#version 330 core
out vec4 FragColor;

in vec3 color;
in vec3 fragPos;
in vec3 normal;

uniform vec3 viewPos;
uniform vec3 lightPos;

void main()
{
   vec3 lightColor = vec3(0.7, 0.7, 0.7);

   // ambient light
   float ambientStrength = 0.7;
   vec3 ambient = ambientStrength * lightColor;

   // diffuse light
   vec3 norm = normalize(-normal);
   vec3 lightDir = normalize(lightPos - fragPos);
   float diff = max(dot(norm, lightDir), 0.0);
   vec3 diffuse = diff * lightColor;

   // specular light
   float specularStrength = 0.25;
   vec3 viewDir = normalize(viewPos - fragPos);
   vec3 reflectDir = reflect(-lightDir, norm);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), 16);
   vec3 specular = specularStrength * spec * lightColor;

   vec3 result = (ambient + diffuse + specular) * color;

   if (fragPos.y > -15.0) {
      vec3 result = (ambient + diffuse + specular) * color * (1.0 + fragPos.y / 15.0) + vec3(0.0) * (-fragPos.y) / 15.0;
      FragColor = vec4(result, 1.0);
   }
   else {
      FragColor = vec4(vec3(0.0), 1.0);
   }
}