#version 460 core

in vec2 color;

out vec4 final_color;

void main() {

  final_color = vec4(color.x, color.y, 0.0, 1.0);
}
