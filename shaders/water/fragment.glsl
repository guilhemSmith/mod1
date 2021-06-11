#version 400 core
out vec4 FragColor;

in vec3 fragPos;
in vec3 normal;

uniform sampler2D screenTexture;
uniform vec3 viewPos;
uniform vec3 lightPos;
uniform vec2 viewportRes;
uniform int time;

vec3 light_color(vec3 base_color, vec3 normal, vec3 fragPos, vec3 viewPos, vec3 lightPos, float specularStrength, int shininess);

float linearize_depth(float zoverw){
   float n = 0.1; // camera z near
   float f = 300.0; // camera z far
   return (2.0 * n) / (f + n - zoverw * (f - n));
}

void main()
{
   vec2 uv = gl_FragCoord.xy / viewportRes;
   float depth = texture(screenTexture, uv).r;
	depth = linearize_depth(depth);
   float height = linearize_depth(gl_FragCoord.z);
   float delta = depth - height;
   float depth_fade = exp(-delta * 10.0);
   depth_fade = clamp(depth_fade, 0.0, 1.0);
   vec3 deep_color = vec3(0.05, 0.15, 0.15);
   vec3 shallow_color = vec3(0.1, 0.5, 0.6);
   vec3 water_color = deep_color * (1.0 - depth_fade) + shallow_color * depth_fade;
   if (delta > 0.005 || normal.y == 0.0) {
      vec3 final = clamp(light_color(water_color, normal, fragPos, viewPos, lightPos, 0.5, 32), 0.0, 1.0);
      FragColor = vec4(final, 1.0);
   }
   else {
      delta = clamp(delta * 200.0, 0.0, 1.0);
      vec4 foam_color = vec4(vec3(1.0), 0.5);
      vec4 color = foam_color * (1.0 - delta) + vec4(water_color, 1.0) * delta;
      FragColor = clamp(vec4(light_color(color.xyz, normal, fragPos, viewPos, lightPos, 0.5, 32), color.a), 0.0, 1.0);
   }
}