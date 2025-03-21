attribute vec4 in_pos;
attribute vec2 in_uv;
attribute vec3 in_norm;

uniform mat4 mvp;
uniform vec3 lightPosModelSpace;

varying vec2 uv;
varying float rmp;

void main() {
  uv = in_uv;
  rmp = 0.35 - 0.35 * dot(in_norm, normalize(in_pos.xyz - lightPosModelSpace));
  gl_Position = mvp * in_pos;
}
