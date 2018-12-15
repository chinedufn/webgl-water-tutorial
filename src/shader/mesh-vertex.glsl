attribute vec3 position;
attribute vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

varying vec3 vNormal;
varying float shouldClip;

const vec4 clipPlane = vec4(0.0, -1.0, 0.0, -0.5);

void main (void) {
  vec4 worldPosition = model * vec4(position, 1.0);

  vNormal = normal;

  shouldClip = dot(worldPosition, clipPlane);

  gl_Position = perspective * view * worldPosition;
}
