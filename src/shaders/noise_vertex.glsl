#version 460 core

layout (location = 0) in vec2 pos;

out vec2 color;

uniform mat3 transform;

void main() {
  color = (pos + 1) / 2;
  gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}
