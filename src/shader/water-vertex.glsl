attribute vec3 position;

uniform mat4 perspective;
uniform mat4 modelView;

varying vec4 clipSpace;

// TODO: Breadcrumb - projective texture mapping of refraction an reflection textures
void main() {
    clipSpace = perspective * modelView * vec4(position, 1.0);

    gl_Position = clipSpace;
}
