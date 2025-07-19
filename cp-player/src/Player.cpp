#include <stdexcept>
#include <iostream>
#include <vector>

#include "Player.hpp"

// --- Shader Source Code ---
// A simple Vertex Shader
const char* vertexShaderSource = R"(
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
)";

// A simple Fragment Shader
const char* fragmentShaderSource = R"(
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f); // Orange color
    }
)";


void error_callback(int error, const char* description) {
    fprintf(stderr, "Error: %s\n", description);
}

// Helper function to check for shader compilation errors
void checkShaderCompilation(GLuint shader) {
    int success;
    char infoLog[512];
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        glGetShaderInfoLog(shader, 512, NULL, infoLog);
        throw std::runtime_error("Shader compilation failed: " + std::string(infoLog));
    }
}

// Helper function to check for shader program linking errors
void checkProgramLinking(GLuint program) {
    int success;
    char infoLog[512];
    glGetProgramiv(program, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(program, 512, NULL, infoLog);
        throw std::runtime_error("Shader program linking failed: " + std::string(infoLog));
    }
}


Player::Player():
    m_window(nullptr),
    m_width(1200),
    m_height(800),
    m_playing(false),
    m_triangleVao(0),
    m_triangleVbo(0),
    m_shaderProgram(0) {
    glfwSetErrorCallback(error_callback);
    if (!glfwInit()) {
        throw std::runtime_error("Failed to initialize GLFW3");
    }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
}

Player::~Player() {
    // Clean up OpenGL resources
    glDeleteVertexArrays(1, &m_triangleVao);
    glDeleteBuffers(1, &m_triangleVbo);
    glDeleteProgram(m_shaderProgram);

    glfwTerminate();
}

void Player::setup_triangle() {
    // 1. Define triangle vertices
    float vertices[] = {
        -0.5f, -0.5f, 0.0f, // left
         0.5f, -0.5f, 0.0f, // right
         0.0f,  0.5f, 0.0f  // top
    };

    // 2. Create and compile shaders
    GLuint vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    checkShaderCompilation(vertexShader);

    GLuint fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
    glCompileShader(fragmentShader);
    checkShaderCompilation(fragmentShader);

    // 3. Link shaders into a shader program
    m_shaderProgram = glCreateProgram();
    glAttachShader(m_shaderProgram, vertexShader);
    glAttachShader(m_shaderProgram, fragmentShader);
    glLinkProgram(m_shaderProgram);
    checkProgramLinking(m_shaderProgram);

    // Shaders are linked, we can delete them now
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    // 4. Create VAO and VBO
    glGenVertexArrays(1, &m_triangleVao);
    glGenBuffers(1, &m_triangleVbo);

    // Bind the VAO first, then bind and set VBO and attribute pointers
    glBindVertexArray(m_triangleVao);

    glBindBuffer(GL_ARRAY_BUFFER, m_triangleVbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    // Tell OpenGL how to interpret the vertex data
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
    glEnableVertexAttribArray(0);

    // Unbind the VBO and VAO
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    glBindVertexArray(0);
}


void Player::render_triangle() {
    // Use our shader program
    glUseProgram(m_shaderProgram);
    // Bind the VAO for our triangle
    glBindVertexArray(m_triangleVao);
    // Draw the triangle
    glDrawArrays(GL_TRIANGLES, 0, 3);
}


void Player::start() {
    m_window = glfwCreateWindow(m_width, m_height, "CP Player", nullptr, nullptr);
    if (!m_window) {
        throw std::runtime_error("Failed to create GLFW window");
    }

    glfwMakeContextCurrent(m_window);

    if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
        throw std::runtime_error("Failed to initialize GLAD");
    }

    // --- Set up our triangle ---
    setup_triangle();

    // Set a background color
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);

    while (!glfwWindowShouldClose(m_window)) {
        glfwPollEvents();

        glfwGetWindowSize(m_window, &m_width, &m_height);
        glViewport(0, 0, m_width, m_height);

        // Clear the screen
        glClear(GL_COLOR_BUFFER_BIT);

        // --- Render the triangle ---
        render_triangle();

        glfwSwapBuffers(m_window);
    }

    glfwDestroyWindow(m_window);
}