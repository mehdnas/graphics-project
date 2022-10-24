#version 460 core

in vec2 tex_coords;
in vec2 pos;

uniform sampler2D screen_texture;

out vec4 final_color;

void main() {

  bool is_axis = abs(pos.x) < 1.0 / 800 || abs(pos.y) < 1.0 / 600;
  vec4 axis_color = mix(
    texture(screen_texture, tex_coords),
    vec4(0.0, 0.0, 1.0, 1.0),
    0.7
  );
  final_color = float(!is_axis) * texture(screen_texture, tex_coords)
              + float(is_axis) * axis_color;
}
