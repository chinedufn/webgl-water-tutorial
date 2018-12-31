precision mediump float;

uniform sampler2D refractionTexture;
uniform sampler2D reflectionTexture;
uniform sampler2D dudvTexture;
uniform sampler2D normalMap;
uniform sampler2D waterDepthTexture;

// FIXME: Uniforms .. to avoid accidentally having different values across shaders
vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
vec3 sunlightDir = normalize(vec3(-1.0, -1.0, 0.5));

varying vec3 fromFragmentToCamera;

// Changes over time, making the water look like it's moving
uniform float dudvOffset;

varying vec4 clipSpace;

varying vec2 textureCoords;

const float waterDistortionStrength = 0.03;
const float shineDamper = 20.0;

uniform float waterReflectivity;
uniform float fresnelStrength;

vec4 shallowWaterColor =  vec4(0.0, 0.1, 0.3, 1.0);
vec4 deepWaterColor = vec4(0.0, 0.1, 0.2, 1.0);

vec3 getNormal(vec2 textureCoords);



void main() {
    // Between 0 and 1
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    float near = 0.1;
    float far = 50.0;

    float cameraToFloorDepth = texture2D(waterDepthTexture, refractTexCoords).r;
    // FIXME: Understand what's going on here
    float floorDistance = 2.0 * near * far / (far + near - (2.0 * cameraToFloorDepth - 1.0) * (far - near));

    float cameraToWaterDepth = gl_FragCoord.z;
    float cameraToWaterDistance = 2.0 * near * far / (far + near - (2.0 * cameraToWaterDepth - 1.0) * (far - near));

    float waterToFloor = floorDistance - cameraToWaterDistance;

    vec2 distortedTexCoords = texture2D(dudvTexture, vec2(textureCoords.x + dudvOffset, textureCoords.y)).rg * 0.1;
    distortedTexCoords = textureCoords + vec2(distortedTexCoords.x, distortedTexCoords.y + dudvOffset);

    float fullOpacityDepth = 0.005;

    // Between -1 and 1
    vec2 totalDistortion = (texture2D(dudvTexture, distortedTexCoords).rg * 2.0 - 1.0)
     * waterDistortionStrength
     * clamp(waterToFloor / (fullOpacityDepth * 6.0), 0.0, 1.0);

    refractTexCoords += totalDistortion;
    reflectTexCoords += totalDistortion;

    refractTexCoords = clamp(refractTexCoords, 0.001, 0.999);
    reflectTexCoords.x = clamp(reflectTexCoords.x, 0.001, 0.999);
    reflectTexCoords.y = clamp(reflectTexCoords.y, -0.999, -0.001);

    vec4 reflectColor = texture2D(reflectionTexture, reflectTexCoords);

    vec4 refractColor = texture2D(refractionTexture, refractTexCoords);
    refractColor = mix(refractColor, deepWaterColor, clamp(waterToFloor/10.0, 0.0, 1.0));

    vec3 toCamera = normalize(fromFragmentToCamera);

    vec3 normal = getNormal(distortedTexCoords);

    // Fresnel Effect. Looking at the water from above makes the water more transparent.
    float refractiveFactor = dot(toCamera, normal);

    // A higher fresnelStrength makes the water more reflective since the
    // refractive factor will decrease
    refractiveFactor = pow(refractiveFactor, fresnelStrength);

    vec3 reflectedLight = reflect(normalize(sunlightDir), normal);
    float specular = max(dot(reflectedLight, toCamera), 0.0);
    specular = pow(specular, shineDamper);
    vec3 specularHighlights = sunlightColor * specular * waterReflectivity
        * clamp(waterToFloor / (fullOpacityDepth * 6.0), 0.0, 1.0);

    gl_FragColor = mix(reflectColor, refractColor, refractiveFactor);
    // Mix in a bit of blue so that it looks like water
    gl_FragColor = mix(gl_FragColor, shallowWaterColor, 0.2) + vec4(specularHighlights, 0.0);

    gl_FragColor.a = clamp(waterToFloor / fullOpacityDepth, 0.0, 1.0);
}

vec3 getNormal(vec2 textureCoords) {
    vec4 normalMapColor = texture2D(normalMap, textureCoords);
    float makeNormalPointUpwardsMore = 2.6;
    vec3 normal = vec3(
      normalMapColor.r * 2.0 - 1.0,
      normalMapColor.b * makeNormalPointUpwardsMore,
      normalMapColor.g * 2.0 - 1.0
    );
    normal = normalize(normal);

    return normal;
}
