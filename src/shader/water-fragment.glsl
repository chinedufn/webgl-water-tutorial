precision mediump float;

uniform sampler2D refractionTexture;
uniform sampler2D reflectionTexture;
uniform sampler2D dudvTexture;
uniform sampler2D normalMap;

varying vec3 fromFragmentToCamera;

// Changes over time, making the water look like it's moving
uniform float dudvOffset;

varying vec4 clipSpace;

varying vec2 textureCoords;

const float distortionStrength = 0.02;

void main() {
    // FIXME: Calculate in vertex shader
    // Between 0 and 1
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    // Between 0 and 1
    vec2 distortion1 = texture2D(
        dudvTexture,
        vec2(
            textureCoords.x + dudvOffset,
            textureCoords.y
        )
    ).rg;

    // Between 0 and 1
    vec2 distortion2 = texture2D(
        dudvTexture,
        vec2(
            -textureCoords.x + dudvOffset,
            textureCoords.y + dudvOffset
        )
    ).rg;

    // Between -1 and 1
    vec2 totalDistortion = (distortion1 + distortion2) - 1.0;

    totalDistortion = totalDistortion * distortionStrength;

    refractTexCoords += totalDistortion;
    reflectTexCoords += totalDistortion;

    refractTexCoords = clamp(refractTexCoords, 0.001, 0.999);
    reflectTexCoords.x = clamp(reflectTexCoords.x, 0.001, 0.999);
    reflectTexCoords.y = clamp(reflectTexCoords.y, -0.999, -0.001);

    vec4 refractColor = texture2D(refractionTexture, refractTexCoords);
    vec4 reflectColor = texture2D(reflectionTexture, reflectTexCoords);

    vec3 toCamera = normalize(fromFragmentToCamera);

    // Fresnel Effect. Looking at the water from above makes the water more transparent.
    float refractiveFactor = dot(toCamera, vec3(0.0, 1.0, 0.0));

    // A higher power makes the water more reflective since the refractive factor will decrease
    // FIXME: Control refractiveFactor with a slider. Call it the fresnel effect slider
    refractiveFactor = pow(refractiveFactor, 1.5);

    gl_FragColor = mix(reflectColor, refractColor, refractiveFactor);
    // Mix in a bit of blue so that it looks like water
    gl_FragColor = mix(gl_FragColor, vec4(0.0, 0.3, 0.5, 1.0), 0.2);
}
