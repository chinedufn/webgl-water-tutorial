precision mediump float;

uniform sampler2D refractionTexture;
uniform sampler2D reflectionTexture;
uniform sampler2D dudvTexture;

// Changes over time, making the water look like it's moving
uniform float dudvOffset;

varying vec4 clipSpace;

varying vec2 textureCoords;

const float distortionStrength = 0.02;

void main() {
    // FIXME: Calculate in vertex shader
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    vec2 distortion1 = texture2D(
        dudvTexture,
        vec2(
            textureCoords.x + dudvOffset,
            textureCoords.y
        )
    ).rg;

    vec2 distortion2 = texture2D(
        dudvTexture,
        vec2(
            -textureCoords.x + dudvOffset,
            textureCoords.y + dudvOffset
        )
    ).rg;

    vec2 totalDistortion = (distortion1 + distortion2) / 2.0 - 1.0;

    totalDistortion = totalDistortion * distortionStrength;

    refractTexCoords += totalDistortion;
    reflectTexCoords += totalDistortion;

    clamp(refractTexCoords, 0.001, 0.999);
    clamp(reflectTexCoords.x, 0.001, 0.999);
    clamp(reflectTexCoords.y, -0.999, -0.001);

    vec4 refraction = texture2D(refractionTexture, refractTexCoords);
    vec4 reflection = texture2D(reflectionTexture, reflectTexCoords);

    gl_FragColor = mix(refraction, reflection, 0.5);
    // Mix in a bit of blue so that it looks like water
    gl_FragColor = mix(gl_FragColor, vec4(0.0, 0.3, 0.5, 1.0), 0.2);
}
