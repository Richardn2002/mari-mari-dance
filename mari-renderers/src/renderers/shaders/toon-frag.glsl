varying vec2 uv;
varying float rmp;

uniform sampler2D tex;
uniform sampler2D rmp_tex;
uniform sampler2D sdw_tex;

void main() {
  vec3 rmpCoeff;
  if(texture2D(sdw_tex, uv).a > 0.5) {
    rmpCoeff = texture2D(rmp_tex, vec2(rmp, 0)).rgb;
  } else {
    rmpCoeff = vec3(1.0, 1.0, 1.0);
  }

  vec3 col = texture2D(tex, uv).rgb;
  gl_FragColor = vec4(col * rmpCoeff, 1);
}
