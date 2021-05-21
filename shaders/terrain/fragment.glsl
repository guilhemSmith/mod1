#version 330 core
out vec4 FragColor;

in vec3 color;
in vec3 fragPos;
in vec3 normal;

uniform vec3 viewPos;

void main()
{
   vec3 lightColor = vec3(0.8);
   vec3 lightPos = vec3(150.0, -150.0, 150.0);

   float ambientStrength = 0.5;
   vec3 ambient = ambientStrength * lightColor;

   vec3 norm = normalize(normal);
   vec3 lightDir = normalize(lightPos - fragPos);

   float diff = max(dot(norm, lightDir), 0.0);
   vec3 diffuse = diff * lightColor;

   float specularStrength = 0.25;
   vec3 viewDir = normalize(viewPos - fragPos);
   vec3 reflectDir = reflect(-lightDir, norm);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), 4);
   vec3 specular = specularStrength * spec * lightColor;

   vec3 result = (ambient + diffuse + specular) * color;
   FragColor = vec4(result, 1.0);
}