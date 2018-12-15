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

    gl_FragColor = vec4(vNormal, 1.0);
}
