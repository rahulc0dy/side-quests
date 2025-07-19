#pragma once

#include <optional>
#include <stdexcept>
#include <iostream>


#include "glad/glad.h"
#include "GLFW/glfw3.h"

class Player {
public:
    Player();
    ~Player();

    void start();

private:
    void setup_triangle();
    void render_triangle();

private:
    GLFWwindow* m_window;
    GLsizei m_width, m_height;
    bool m_playing;

    GLuint m_triangleVao;
    GLuint m_triangleVbo;
    GLuint m_shaderProgram;
};
