#pragma once
#include <string>
#include <glad/glad.h>
#include <fstream>
#include <sstream>
#include <iostream>
#include "tinyfiledialogs.h"

class ShaderManager {
public:
    ShaderManager();
    ~ShaderManager();

    bool loadShaders(const std::string& vertexPath, const std::string& fragmentPath, std::string& errorLog);
    bool loadShadersFromStrings(const std::string& vertSrc, const std::string& fragSrc, std::string& errorLog);
    std::string openFileDialog(const char* filter = "*.vert;*.frag");
    std::string readFile(const std::string& filePath);
    void use();
    [[nodiscard]] GLuint program() const;

    // Uniform helpers
    void setUniform(const std::string& name, float value);
    void setUniform(const std::string& name, int value);
    void setUniform(const std::string& name, float v0, float v1);
    void setUniform(const std::string& name, float v0, float v1, float v2, float v3);

private:
    GLuint m_program;
    GLuint m_vertexShader;
    GLuint m_fragmentShader;
    void cleanup();
    std::string loadFile(const std::string& path);
    GLuint compileShader(GLenum type, const std::string& src, std::string& errorLog);
};