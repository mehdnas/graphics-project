#version 460 core

in vec2 frag_pos;


uniform float m;
uniform float b;
uniform vec4 line_color;

out vec4 final_color;

void main() {

  if (round(frag_pos.y) == round(m * frag_pos.x + b)) {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
  else {
    discard;
  }
}
