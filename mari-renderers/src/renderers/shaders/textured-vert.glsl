attribute vec3 in_pos;
attribute vec2 in_uv;
uniform mat4 mvp;
varying vec2 uv;
void main() {
    gl_Position = mvp * vec4(in_pos, 1.0);
    uv = in_uv;
}