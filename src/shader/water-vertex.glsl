attribute vec2 position;

uniform mat4 perspective;
uniform mat4 model;
uniform mat4 view;

uniform vec3 cameraPos;
varying vec3 fromFragmentToCamera;

varying vec4 clipSpace;
varying vec2 textureCoords;

const float tiling = 4.0;

void main() {
    vec4 worldPosition = model * vec4(position.x, 0.0, position.y, 1.0);

    clipSpace = perspective * view *  worldPosition;

    gl_Position = clipSpace;

    // (-0.5 < pos < 0.5) -> (0.0 < pos < 1.0)
    textureCoords = position + 0.5;
    textureCoords = textureCoords * tiling;

    fromFragmentToCamera = cameraPos - worldPosition.xyz;
}
