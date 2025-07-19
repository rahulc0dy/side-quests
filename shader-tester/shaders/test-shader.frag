#version 460 core

uniform float iTime;
uniform vec2 iResolution;
out vec4 FragColor;

void mainImage(out vec4 O, vec2 C) {
    float i = 0.0; // Loop counter
    float d;       // Distance to nearest surface
    float z = fract(dot(C, sin(C))) - .5; // Ray distance + noise for anti-banding

    vec4 o = vec4(0.0); // Accumulated color/lighting
    vec4 p;             // Current 3D position along ray

    for (
    vec2 r = iResolution.xy; ++i < 77.0; z += .6*d // Step forward
    ) {
        // Convert 2D pixel to 3D ray direction
        p = vec4(z * normalize(vec3(C - 0.5 * r, r.y)), .1 * iTime);

        // Move through 3D space over time
        p.z += iTime;

        // Save position for lighting calculations
        O = p;

        // Apply rotation matrices to create fractal patterns
        p.xy *= mat2(cos(2.0 + O.z + vec4(0, 11, 33, 0)));
        p.xy *= mat2(cos(O + vec4(0, 11, 33, 0)));

        // Calculate color based on position and space distortion
        O = (1.0 + sin(.5 * O.z + length(p - O) + vec4(0, 4, 3, 6)))
        / (.5 + 2.0 * dot(O.xy, O.xy));

        // Domain repetition
        p = abs(fract(p) - .5);

        // Calculate distance to nearest surface
        d = abs(min(length(p.xy) - .125, min(p.x, p.y) + 1e-3)) + 1e-3;

        // Add lighting contribution
        o += O.w / d * O;
    }
    // tanh() compresses the accumulated brightness to 0-1 range
    O = tanh(o / 2e4);
}

void main() {
    mainImage(FragColor, gl_FragCoord.xy);
}