#pragma once

// Default vertex shader
inline const char* DEFAULT_VERTEX_SHADER = R"(#version 460 core
layout(location = 0) in vec2 aPos;
void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
}
)";

// Default fragment shader
inline const char* DEFAULT_FRAGMENT_SHADER = R"(#version 460 core
uniform float iTime;
uniform vec2 iResolution;
uniform vec4 iMouse;

out vec4 FragColor;

void main() {
    vec2 uv = gl_FragCoord.xy / iResolution.xy;
    vec3 col = 0.5 + 0.5 * cos(iTime + uv.xyx + vec3(0,2,4));
    FragColor = vec4(col, 1.0);
}
)";