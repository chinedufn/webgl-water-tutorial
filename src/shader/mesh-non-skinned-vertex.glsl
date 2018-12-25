attribute vec3 position;
attribute vec3 normal;

attribute vec2 uvs;
varying vec2 vUvs;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

varying vec3 vNormal;
varying vec3 vWorldPos;
varying float shouldClip;

uniform vec3 cameraPos;
varying vec3 fromFragmentToCamera;

uniform vec4 clipPlane;

void main (void) {
  vec4 worldPosition = model * vec4(position, 1.0);

  shouldClip = dot(worldPosition, clipPlane) < 0.0 ? 1.0 : 0.0;

  gl_Position = perspective * view * worldPosition;

  vNormal = normal;
  vWorldPos = worldPosition.xyz;
  fromFragmentToCamera = cameraPos - worldPosition.xyz;

  vUvs = uvs;
}
