precision mediump float;

uniform sampler2D refractionTexture;
uniform sampler2D reflectionTexture;

varying vec4 clipSpace;

void main() {
    // FIXME: Calculate in vertex shader
    vec2 ndc = (clipSpace.xy / clipSpace.w) / 2.0 + 0.5;

    vec2 refractTexCoords = vec2(ndc.x, ndc.y);
    // Reflections are upside down
    vec2 reflectTexCoords = vec2(ndc.x, -ndc.y);

    vec4 refraction = texture2D(refractionTexture, refractTexCoords);
    vec4 reflection = texture2D(reflectionTexture, reflectTexCoords);

    gl_FragColor = mix(refraction, reflection, 0.5);
    // Mix in a bit of blue so that it looks like water
    gl_FragColor = mix(gl_FragColor, vec4(0.0, 0.3, 0.5, 1.0), 0.3);
}
