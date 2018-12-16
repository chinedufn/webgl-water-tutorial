precision mediump float;

// TODO: Pass this uniform in
//uniform vec3 uCameraPos;

varying vec3 vNormal;

varying float shouldClip;

// FIXME: Lighting
void main(void) {
    if (shouldClip == 1.0) {
        discard;
    }

    vec3 color = vNormal / 2.0 + 0.5;
    gl_FragColor = vec4(color, 1.0);
}
