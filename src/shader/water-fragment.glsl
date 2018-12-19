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

const float waterDistortionStrength = 0.04;
const float shineDamper = 20.0;
const float lightReflectivity = 0.5;

void main() {
    // FIXME: Calculate in vertex shader
    // Between 0 and 1
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    // FIXME: Uniform
    float near = 0.1;
    // FIXME: Uniform
    float far = 50.0;
    float cameraToFloorDepth = texture2D(waterDepthTexture, refractTexCoords).r;
    // FIXME: Understand what's going on here
    float floorDistance = 2.0 * near * far / (far + near - (2.0 * cameraToFloorDepth - 1.0) * (far - near));

    float cameraToWaterDepth = gl_FragCoord.z;
    float cameraToWaterDistance = 2.0 * near * far / (far + near - (2.0 * cameraToWaterDepth - 1.0) * (far - near));

    float waterToFloor = floorDistance - cameraToWaterDistance;

    // FIXME: Explanation and better name
    // FIXME: Tweak this based on our scene
    // FIXME: Add HTML sliders for a bunch of this stuff
    float fullOpacityDepth = 0.005;

    vec2 distortedTexCoords = texture2D(dudvTexture, vec2(textureCoords.x + dudvOffset, textureCoords.y)).rg * 0.1;
    distortedTexCoords = textureCoords + vec2(distortedTexCoords.x, distortedTexCoords.y + dudvOffset);

    // Between -1 and 1
    vec2 totalDistortion = (texture2D(dudvTexture, distortedTexCoords).rg * 2.0 - 1.0)
     * waterDistortionStrength
     * clamp(waterToFloor / (fullOpacityDepth * 6.0), 0.0, 1.0);
//    totalDistortion = vec2(0.0, 0.0);

    refractTexCoords += totalDistortion;
    reflectTexCoords += totalDistortion;

    refractTexCoords = clamp(refractTexCoords, 0.001, 0.999);
    reflectTexCoords.x = clamp(reflectTexCoords.x, 0.001, 0.999);
    reflectTexCoords.y = clamp(reflectTexCoords.y, -0.999, -0.001);

    vec4 refractColor = texture2D(refractionTexture, refractTexCoords);
    vec4 reflectColor = texture2D(reflectionTexture, reflectTexCoords);

    vec3 toCamera = normalize(fromFragmentToCamera);

    vec4 normalMapColor = texture2D(normalMap, distortedTexCoords);
    vec3 normal = vec3(normalMapColor.r * 2.0 - 1.0, normalMapColor.b * 2.6, normalMapColor.g * 2.0 - 1.0);
    normal = normalize(normal);

    // Fresnel Effect. Looking at the water from above makes the water more transparent.
    float refractiveFactor = dot(toCamera, normal);

    // A higher power makes the water more reflective since the refractive factor will decrease
    // FIXME: Control refractiveFactor with a slider. Call it the fresnel effect slider
    refractiveFactor = pow(refractiveFactor, 1.5);

    vec3 reflectedLight = reflect(normalize(sunlightDir), normal);
    float specular = max(dot(reflectedLight, toCamera), 0.0);
    specular = pow(specular, shineDamper);
    vec3 specularHighlights = sunlightColor * specular * lightReflectivity
        * clamp(waterToFloor / (fullOpacityDepth * 6.0), 0.0, 1.0);

    gl_FragColor = mix(reflectColor, refractColor, refractiveFactor);
    // Mix in a bit of blue so that it looks like water
    gl_FragColor = mix(gl_FragColor, vec4(0.0, 0.3, 0.5, 1.0), 0.2) + vec4(specularHighlights, 0.0);


    // FIXME: Remove all of the alpha and water depth blending stuff and see if it makes a difference in our
    // final scene..
    gl_FragColor.a = clamp(waterToFloor / fullOpacityDepth, 0.0, 1.0);
}
