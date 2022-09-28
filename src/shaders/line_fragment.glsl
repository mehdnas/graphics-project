#version 460 core

in vec2 frag_pos;


uniform float m;
uniform float b;

out vec4 final_color;

void main() {

  if (int(round(frag_pos.y)) == int(round(m * frag_pos.x + b))) {
    final_color = vec4(0.0, 1.0, 0.0, 1.0);
  }
  else {
    discard;
  }
}
