#version 460 core

in vec2 frag_pos;


uniform vec2 point1;
uniform vec2 point2;
uniform vec4 line_color;
uniform vec4 back_color;

out vec4 final_color;

void main() {

  vec2 rounded = round(frag_pos);
  if (rounded == round(point1) || rounded == round(point2)) {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
  else {
    discard;
  }
}
