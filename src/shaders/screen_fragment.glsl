#version 460 core

in vec2 tex_coords;

uniform sampler2D screen_texture;

out vec4 final_color;

void main() {
  final_color = texture(screen_texture, tex_coords);
}
