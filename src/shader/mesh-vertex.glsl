attribute vec3 position;
attribute vec3 normal;

uniform mat4 modelView;
uniform mat4 perspective;

varying vec3 vNormal;

void main (void) {
  gl_Position = perspective * modelView * vec4(position, 1.0);

  vNormal = normal;
}
