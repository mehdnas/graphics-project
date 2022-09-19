#version 460 core

out vec4 final_color;

float random (vec2 st) {
  return fract(sin(dot(
    st.xy, vec2(12.9898, 78.33))) * 43758.5453123
  );
}

void main() {

  vec2 st = gl_FragCoord.xy;
  float rnd = random(st);
  final_color = vec4(vec3(rnd), 1.0);
}
