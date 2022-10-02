#version 460 core

layout (location = 0) in vec2 i_pos;
layout (location = 1) in vec2 i_tex_coords;

out vec2 tex_coords;
out vec2 pos;

uniform mat3 transform;

void main() {
  tex_coords = i_tex_coords;
  pos = i_pos;
  vec3 transformed_pos = transform * vec3(i_pos.x, i_pos.y, 1.0);
  gl_Position = vec4(transformed_pos.x, transformed_pos.y, transformed_pos.z, 1.0);
}
