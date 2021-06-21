#version 400 core
out vec4 FragColor;

in vec3 fragPos;
in vec3 normal;
in vec2 noiseCoord;

uniform sampler2D depthTexture;
uniform sampler2D foamTexture;

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
   float depth = texture(depthTexture, uv).r;
	depth = linearize_depth(depth);
   float height = linearize_depth(gl_FragCoord.z);
   float delta = depth - height;
   float depth_fade = exp(-delta * 10.0);
   depth_fade = clamp(depth_fade, 0.0, 1.0);
   float foam = texture(foamTexture, texture(foamTexture, noiseCoord).gb / 25.0 + noiseCoord + vec2(sin(time * 0.00001), cos(time * 0.00001))).r;
   FragColor = vec4(vec3(texture(foamTexture, noiseCoord).g), 1.0);
   if (normal.y == 0.0 || (delta > 0.005 && foam < 0.5)) {
      vec3 deep_color = vec3(0.05, 0.15, 0.15);
      vec3 shallow_color = vec3(0.1, 0.5, 0.6);
      vec3 water_color = deep_color * (1.0 - depth_fade) + shallow_color * depth_fade;
      vec3 final = clamp(light_color(water_color, normal, fragPos, viewPos, lightPos, 0.5, 32), 0.0, 1.0);
      FragColor = vec4(final, 1.0);
   }
   else {
      vec4 foam_color = vec4(vec3(1.0), 0.5);
      FragColor = clamp(vec4(light_color(foam_color.rgb, normal, fragPos, viewPos, lightPos, 0.5, 32), 0.5), 0.0, 1.0);
   }
}