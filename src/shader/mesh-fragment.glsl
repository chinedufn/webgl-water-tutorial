precision mediump float;

// TODO: Pass this uniform in
//uniform vec3 uCameraPos;

varying vec3 vNormal;

// FIXME: Lighting
void main(void) {
    gl_FragColor = vec4(vNormal, 1.0);
}
