#version 460 core

layout (location = 0) in vec2 pos;

uniform mat3 transform;
uniform vec2 canvas_size;

out vec2 frag_pos;

void main() {
  frag_pos = (pos * canvas_size) / 2;
  gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}
