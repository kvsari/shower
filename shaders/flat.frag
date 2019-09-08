#version 450

// Flat shader.

const int MAX_LIGHTS = 10;

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec3 v_Normal;
layout(location = 2) in vec3 f_Colour;

layout(location = 0) out vec4 o_Colour;

struct Light {
  mat4 projection;
  vec4 position;
  vec4 colour;
};

layout(set = 0, binding = 2) uniform Lights {
  Light u_Lights[2];
};

layout(set = 0, binding = 3) uniform NumberOfLights {
  int u_LightCount;
};

void main() {
  vec3 normal = normalize(v_Normal);
  vec3 ambient = vec3(0.05, 0.05, 0.05);

  vec3 colour = ambient;
  for(int i = 0; i < u_LightCount && i < MAX_LIGHTS; ++i) {
    Light light = u_Lights[i];
    vec4 light_local = light.projection * v_Position;
    vec3 light_dir = normalize(light.position.xyz - v_Position.xyz);
    float diffuse = max(0.0, dot(normal, light_dir));
    colour += diffuse * light.colour.xyz;
  }
  
  o_Colour = vec4(colour, 1.0) * vec4(f_Colour, 1.0);
}
