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

attribute vec4 jointIndices;
attribute vec4 jointWeights;

uniform vec3 ambientColor;

uniform vec4 boneRotQuaternions[20];
uniform vec4 boneTransQuaternions[20];

void main (void) {
  // Blend our dual quaternion
  vec4 weightedRotQuats = boneRotQuaternions[int(jointIndices.x)] * jointWeights.x +
    boneRotQuaternions[int(jointIndices.y)] * jointWeights.y +
    boneRotQuaternions[int(jointIndices.z)] * jointWeights.z +
    boneRotQuaternions[int(jointIndices.w)] * jointWeights.w;

  vec4 weightedTransQuats = boneTransQuaternions[int(jointIndices.x)] * jointWeights.x +
    boneTransQuaternions[int(jointIndices.y)] * jointWeights.y +
    boneTransQuaternions[int(jointIndices.z)] * jointWeights.z +
    boneTransQuaternions[int(jointIndices.w)] * jointWeights.w;

  // Normalize our dual quaternion (necessary for nlerp)
  float wRot = weightedRotQuats[0];
  float xRot = weightedRotQuats[1];
  float yRot = weightedRotQuats[2];
  float zRot = weightedRotQuats[3];
  float magnitude = sqrt(xRot * xRot + yRot * yRot + zRot * zRot + wRot * wRot);
  weightedRotQuats = weightedRotQuats / magnitude;
  weightedTransQuats = weightedTransQuats / magnitude;

  // Convert out dual quaternion in a 4x4 matrix
  //  equation: https://www.cs.utah.edu/~ladislav/kavan07skinning/kavan07skinning.pdf
  float wR = weightedRotQuats[0];
  float xR = weightedRotQuats[1];
  float yR = weightedRotQuats[2];
  float zR = weightedRotQuats[3];

  float wT = weightedTransQuats[0];
  float xT = weightedTransQuats[1];
  float yT = weightedTransQuats[2];
  float zT = weightedTransQuats[3];

  float t0 = 2.0 * (-wT * xR + xT * wR - yT * zR + zT * yR);
  float t1 = 2.0 * (-wT * yR + xT * zR + yT * wR - zT * xR);
  float t2 = 2.0 * (-wT * zR - xT * yR + yT * xR + zT * wR);

  mat4 convertedMatrix = mat4(
      1.0 - (2.0 * yR * yR) - (2.0 * zR * zR),
      (2.0 * xR * yR) + (2.0 * wR * zR),
      (2.0 * xR * zR) - (2.0 * wR * yR),
      0,
      (2.0 * xR * yR) - (2.0 * wR * zR),
      1.0 - (2.0 * xR * xR) - (2.0 * zR * zR),
      (2.0 * yR * zR) + (2.0 * wR * xR),
      0,
      (2.0 * xR * zR) + (2.0 * wR * yR),
      (2.0 * yR * zR) - (2.0 * wR * xR),
      1.0 - (2.0 * xR * xR) - (2.0 * yR * yR),
      0,
      t0,
      t1,
      t2,
      1
      );

  // Transform our normal using our blended transformation matrix.
  // We do not need to take the inverse transpose here since dual quaternions
  // guarantee that we have a rigid transformation matrix.

  // In other words, we know for a fact that there is no scale or shear,
  // so we do not need to create an inverse transpose matrix to account for scale and shear
  vec3 transformedNormal = (convertedMatrix * vec4(normal, 0.0)).xyz;

  // Swap our normal's y and z axis since Blender uses a right handed coordinate system
  float y;
  float z;
  y = transformedNormal.z;
  z = -transformedNormal.y;
  transformedNormal.y = y;
  transformedNormal.z = z;

  // Blender uses a right handed coordinate system. We convert to left handed here
  vec4 leftModelSpacePos = convertedMatrix * vec4(position, 1.0);
  y = leftModelSpacePos.z;
  z = -leftModelSpacePos.y;
  leftModelSpacePos.y = y;
  leftModelSpacePos.z = z;

  vec4 leftWorldSpace = model * leftModelSpacePos;

  gl_Position = perspective * view * leftWorldSpace;

  shouldClip = dot(leftWorldSpace, clipPlane) < 0.0 ? 1.0 : 0.0;
  vNormal = transformedNormal;
  vWorldPos = leftWorldSpace.xyz;
  vUvs = uvs;
  fromFragmentToCamera = cameraPos - leftWorldSpace.xyz;
}
