#version 450

// Flat shader.

layout(location = 0) in vec3 i_Position;
layout(location = 1) in vec3 i_Normal;
layout(location = 2) in vec3 i_Colour;
layout(location = 0) out vec4 v_Position;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec3 f_Colour;

layout(set = 0, binding = 0) uniform Projection {
  mat4 u_Camera;
};

layout(set = 0, binding = 1) uniform Translate {
  mat4 u_Rotation;
};

void main() {  
  v_Position = u_Rotation * vec4(i_Position, 1.0);
  v_Normal = mat3(u_Rotation) * i_Normal;
  f_Colour = i_Colour;
  gl_Position = u_Camera * v_Position;
  gl_Position.z = 0.5 * (gl_Position.z + gl_Position.w);
}
