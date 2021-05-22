#version 400 core
out vec4 FragColor;

in vec3 fragPos;
in vec3 normal;

uniform sampler2D screenTexture;
uniform vec3 viewPos;

float linearize_depth(float zoverw){
   float n = 0.1; // camera z near
   float f = 300.0; // camera z far
   return (2.0 * n) / (f + n - zoverw * (f - n));
}

vec3 light_color(vec3 base_color) {

   vec3 lightColor = vec3(0.7, 0.7, 0.7);
   vec3 lightPos = vec3(150.0, -150.0, 150.0);

   float ambientStrength = 0.7;
   vec3 ambient = ambientStrength * lightColor;

   vec3 norm = normalize(normal);
   vec3 lightDir = normalize(lightPos - fragPos);

   float diff = max(dot(norm, lightDir), 0.0);
   vec3 diffuse = diff * lightColor;

   float specularStrength = 0.25;
   vec3 viewDir = normalize(viewPos - fragPos);
   vec3 reflectDir = reflect(-lightDir, norm);
   float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
   vec3 specular = specularStrength * spec * lightColor;

   return (ambient + diffuse + specular) * base_color;
}

void main()
{
   vec2 uv = gl_FragCoord.xy / vec2(1280.0, 720.0);
   float depth = texture(screenTexture, uv).r;
	depth = linearize_depth(depth);
   float height = linearize_depth(gl_FragCoord.z);
   float delta = depth - height;
   float depth_fade = exp(-delta * 10.0);
   depth_fade = clamp(depth_fade, 0.0, 1.0);
   vec3 deep_color = vec3(0.05, 0.15, 0.15);
   vec3 shallow_color = vec3(0.1, 0.5, 0.6);
   vec3 water_color = deep_color * (1.0 - depth_fade) + shallow_color * depth_fade;
   if (delta > 0.005) {
      vec3 final = clamp(light_color(water_color), 0.0, 1.0);
      FragColor = vec4(final, 1.0);
   }
   else {
      delta = clamp(delta * 200.0, 0.0, 1.0);
      vec4 foam_color = vec4(vec3(1.0), 0.1);
      vec4 color = foam_color * (1.0 - delta) + vec4(water_color, 1.0) * delta;
      FragColor = clamp(vec4(light_color(color.xyz), color.a), 0.0, 1.0);
   }
}