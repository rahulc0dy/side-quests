#include "glad/glad.h"
#include "GLFW/glfw3.h"
#include "ShaderManager.hpp"
#define IMGUI_DEFINE_MATH_OPERATORS
#include "imgui.h"
#include "imgui_internal.h"
#include "backends/imgui_impl_glfw.h"
#include "backends/imgui_impl_opengl3.h"
#include "DefaultShaders.hpp"

#include <filesystem>
#include <iostream>
#include <vector>
#include <string>

void framebuffer_size_callback(GLFWwindow* window, int width, int height) {
    glViewport(0, 0, width, height);
}

int main() {
    if (!glfwInit()) { std::cerr << "GLFW init failed\n"; return -1; }
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* window = glfwCreateWindow(1280, 720, "C++ Shader Tester", nullptr, nullptr);
    if (!window) { std::cerr << "Window failed\n"; glfwTerminate(); return -1; }
    glfwMakeContextCurrent(window);
    glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

    if (!gladLoadGLLoader(reinterpret_cast<GLADloadproc>(glfwGetProcAddress))) {
        std::cerr << "GLAD failed\n"; return -1;
    }
    glViewport(0, 0, 1280, 720);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    const ImGuiIO& io = ImGui::GetIO(); (void)io;
    ImGui::StyleColorsDark();
    ImGui_ImplGlfw_InitForOpenGL(window, true);
    ImGui_ImplOpenGL3_Init("#version 330 core");

    constexpr float quadVerts[] = { -1,-1, 1,-1, -1,1, 1,1 };
    GLuint vao, vbo;
    glGenVertexArrays(1, &vao); glGenBuffers(1, &vbo);
    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(quadVerts), quadVerts, GL_STATIC_DRAW);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 2 * sizeof(float), static_cast<void *>(nullptr));

    ShaderManager shaderMgr;
    std::string vertPath;
    std::string fragPath;
    std::string errorLog;
    bool shaderLoaded = shaderMgr.loadShadersFromStrings(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER, errorLog);
    bool useEmbeddedDefault = true;

    const double startTime = glfwGetTime();
    ImVec2 mousePos(0,0);
    bool mouseDown = false;

    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplGlfw_NewFrame();
        ImGui::NewFrame();

        ImGui::Begin("Shader Tester Controls", nullptr, ImGuiWindowFlags_AlwaysAutoResize);

        if (ImGui::Checkbox("Use Default Shaders", &useEmbeddedDefault)) {
            if (useEmbeddedDefault) {
                shaderLoaded = shaderMgr.loadShadersFromStrings(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER, errorLog);
            } else if (!vertPath.empty() && !fragPath.empty()) {
                shaderLoaded = shaderMgr.loadShaders(vertPath, fragPath, errorLog);
            }
        }

        if (!useEmbeddedDefault) {
            ImGui::Text("Vertex Shader: %s", vertPath.empty() ? "(none selected)" : vertPath.c_str());
            ImGui::SameLine();
            if (ImGui::Button("Select Vertex Shader")) {
                vertPath = shaderMgr.openFileDialog();
            }

            ImGui::Text("Fragment Shader: %s", fragPath.empty() ? "(none selected)" : fragPath.c_str());
            ImGui::SameLine();
            if (ImGui::Button("Select Fragment Shader")) {
                fragPath = shaderMgr.openFileDialog();
            }
        } else {
            ImGui::Text("Using built-in default shaders.");
        }

        if (ImGui::Button("Load/Reload Shaders")) {
            if (useEmbeddedDefault) {
                shaderLoaded = shaderMgr.loadShadersFromStrings(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER, errorLog);
            } else if (!vertPath.empty() && !fragPath.empty()) {
                shaderLoaded = shaderMgr.loadShaders(vertPath, fragPath, errorLog);
            }
        }

        if (!shaderLoaded && !errorLog.empty()) {
            ImGui::TextColored(ImVec4(1,0.2f,0.2f,1), "Shader Error:\n%s", errorLog.c_str());
        }
        ImGui::End();

        int displayW, displayH;
        glfwGetFramebufferSize(window, &displayW, &displayH);
        const float iTime = static_cast<float>(glfwGetTime() - startTime);
        const float iResolution[2] = { static_cast<float>(displayW), static_cast<float>(displayH) };

        mousePos = ImGui::GetMousePos();
        mouseDown = ImGui::IsMouseDown(0);

        glClearColor(0.09f, 0.10f, 0.12f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        if (shaderLoaded) {
            shaderMgr.use();
            shaderMgr.setUniform("iTime", iTime);
            shaderMgr.setUniform("iResolution", iResolution[0], iResolution[1]);
            shaderMgr.setUniform("iMouse", mousePos.x, mousePos.y, mouseDown ? 1.0f : 0.0f, 0.0f);
            glBindVertexArray(vao);
            glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
        }

        ImGui::Render();
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        glfwSwapBuffers(window);
    }

    glDeleteVertexArrays(1, &vao);
    glDeleteBuffers(1, &vbo);
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
    glfwDestroyWindow(window);
    glfwTerminate();
    return 0;
}