precision highp float;

uniform vec2 iResolution;
uniform float iTime;

// Modified from snowfall shader by https://www.shadertoy.com/user/Emil
// Shadertoy: https://www.shadertoy.com/view/Mdt3Df
void main() {
  float snow = 0.0;
  for (int k = 0; k < 3; k++) {
    for (int i = 0; i < 8; i++) {
      float cellSize = floor(2.0 + (float(i) * iResolution.x / 800.));
      float downSpeed =
          0.2 + (sin(iTime * 0.4 + float(k + i * 20)) + 1.0) * 0.00008;
      vec2 uv = (gl_FragCoord.xy / iResolution.x) +
                vec2(0.01 * sin((iTime + float(k * 6185)) * 0.6 + float(i)) *
                         (5.0 / float(i)),
                     downSpeed * (iTime + float(k * 1352)) * (1.0 / float(i)));
      vec2 uvStep = (ceil((uv)*cellSize - vec2(0.5, 0.5)) / cellSize);
      float x = fract(sin(dot(uvStep.xy, vec2(12.9898 + float(k) * 12.0,
                                              78.233 + float(k) * 315.156))) *
                          43758.5453 +
                      float(k) * 12.0) -
                0.5;
      float y = fract(sin(dot(uvStep.xy, vec2(62.2364 + float(k) * 23.0,
                                              94.674 + float(k) * 95.0))) *
                          62159.8432 +
                      float(k) * 12.0) -
                0.5;

      float randomMagnitude1 = sin(iTime * 2.5) * 0.7 / cellSize;
      float randomMagnitude2 = cos(iTime * 2.5) * 0.7 / cellSize;

      float d =
          5.0 * distance((uvStep.xy + vec2(x * sin(y), y) * randomMagnitude1 +
                          vec2(y, x) * randomMagnitude2),
                         uv.xy);

      float omiVal =
          fract(sin(dot(uvStep.xy, vec2(32.4691, 94.615))) * 31572.1684);
      if (omiVal < 0.15) {
        float newd =
            (x + 1.0) * 0.4 *
            clamp(1.9 - d * (15.0 + (x * 6.3)) * (cellSize / 1.4), 0.0, 1.0);
        snow += newd;
      }
    }
  }

  gl_FragColor = vec4(snow);
}
