
#version 460 core

#define PI 3.1415926535897932384626433832795

in vec2 pos;

#define MANDELBROT 0
#define JULIA 1

uniform uint iteration_count;
uniform uint color_jump;
uniform uint fract_type;
uniform vec2 julia_c;

out vec4 final_color;

vec2 complex_sqr(vec2 c) {
  return vec2(c.x * c.x - c.y * c.y, 2 * c.x * c.y);
}

bool is_out_of_bounds(vec2 c) {
  if (length(c) > 4) {
    return true;
  } else {
    return false;
  }
}

const vec2 r_pos = vec2(1, 0);
const vec2 g_pos = vec2(-0.5, 0.86603);
const vec2 b_pos = vec2(-0.5, -0.86603);

vec4 iteration_color(uint i) {
  i = i / color_jump + (iteration_count * (i % color_jump)) / color_jump;
  float arch_i = (2 * PI * float(i)) / float(iteration_count);
  vec2 pos_i = vec2(cos(arch_i), sin(arch_i));
  float r = 1.0 - length(pos_i - r_pos) / 2.0;
  float g = 1.0 - length(pos_i - g_pos) / 2.0;
  float b = 1.0 - length(pos_i - b_pos) / 2.0;
  return vec4(r, g, b, 1.0);
}

void main() {
  vec2 c;
  vec2 z;
  if (fract_type == MANDELBROT) {
    c = pos;
    z = vec2(0, 0);
  } else {
    c = julia_c;
    z = pos;
  }
  bool out_of_bounds = false;
  uint i;
  for (i = 0; i < iteration_count; i++) {
    z = complex_sqr(z) + c;

    out_of_bounds = is_out_of_bounds(z);
    if (out_of_bounds) {
      break;
    }
  }

  if (out_of_bounds) {
    final_color = iteration_color(i);
  } else {
    final_color = vec4(0.0, 0.0, 0.0, 1.0);
  }
}
