#version 400 core

vec3 light_color(vec3 base_color, vec3 normal, vec3 fragPos, vec3 viewPos, vec3 lightPos, float specularStrength, int shininess) {

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
   vec3 viewDir = normalize(viewPos - fragPos);
   vec3 reflectDir = reflect(-lightDir, norm);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
   vec3 specular = specularStrength * spec * lightColor;

   return (ambient + diffuse + specular) * base_color;
}