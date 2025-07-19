#include "ShaderManager.hpp"

ShaderManager::ShaderManager() : m_program(0), m_vertexShader(0), m_fragmentShader(0) {}
ShaderManager::~ShaderManager() { cleanup(); }

std::string ShaderManager::openFileDialog(const char* filter) {
    const char* filterPatterns[] = { "*.vert", "*.frag", "*.glsl" };
    const char* path = tinyfd_openFileDialog(
        "Select Shader", "", 2, filterPatterns, "Shader Files", 0);
    return path ? std::string(path) : std::string();
}

void ShaderManager::cleanup() {
    if (m_program) glDeleteProgram(m_program);
    if (m_vertexShader) glDeleteShader(m_vertexShader);
    if (m_fragmentShader) glDeleteShader(m_fragmentShader);
    m_program = m_vertexShader = m_fragmentShader = 0;
}

std::string ShaderManager::loadFile(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) return "";
    std::stringstream ss; ss << file.rdbuf(); return ss.str();
}

GLuint ShaderManager::compileShader(GLenum type, const std::string& src, std::string& errorLog) {
    GLuint shader = glCreateShader(type);
    const char* cstr = src.c_str();
    glShaderSource(shader, 1, &cstr, nullptr);
    glCompileShader(shader);
    GLint success; glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        char log[1024];
        glGetShaderInfoLog(shader, 1024, nullptr, log);
        errorLog = log;
        glDeleteShader(shader);
        return 0;
    }
    return shader;
}

bool ShaderManager::loadShaders(const std::string& vertexPath, const std::string& fragmentPath, std::string& errorLog) {
    cleanup();
    std::string vertSrc = loadFile(vertexPath);
    std::string fragSrc = loadFile(fragmentPath);
    if (vertSrc.empty() || fragSrc.empty()) {
        errorLog = "Failed to open shader file(s).";
        return false;
    }
    m_vertexShader = compileShader(GL_VERTEX_SHADER, vertSrc, errorLog);
    m_fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragSrc, errorLog);
    if (!m_vertexShader || !m_fragmentShader) return false;
    m_program = glCreateProgram();
    glAttachShader(m_program, m_vertexShader);
    glAttachShader(m_program, m_fragmentShader);
    glLinkProgram(m_program);
    GLint success; glGetProgramiv(m_program, GL_LINK_STATUS, &success);
    if (!success) {
        char log[1024];
        glGetProgramInfoLog(m_program, 1024, nullptr, log);
        errorLog = log;
        cleanup();
        return false;
    }
    return true;
}

std::string ShaderManager::readFile(const std::string& path) {
    std::ifstream file(path);
    std::stringstream ss;
    ss << file.rdbuf();
    return ss.str();
}

bool ShaderManager::loadShadersFromStrings(const std::string &vertSrc, const std::string &fragSrc, std::string &errorLog) {
    cleanup();
    m_vertexShader = compileShader(GL_VERTEX_SHADER, vertSrc, errorLog);
    m_fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragSrc, errorLog);
    if (!m_vertexShader || !m_fragmentShader) return false;
    m_program = glCreateProgram();
    glAttachShader(m_program, m_vertexShader);
    glAttachShader(m_program, m_fragmentShader);
    glLinkProgram(m_program);
    GLint success; glGetProgramiv(m_program, GL_LINK_STATUS, &success);
    if (!success) {
        char log[1024];
        glGetProgramInfoLog(m_program, 1024, nullptr, log);
        errorLog = log;
        cleanup();
        return false;
    }
    return true;
}


void ShaderManager::use() { if (m_program) glUseProgram(m_program); }
GLuint ShaderManager::program() const { return m_program; }

void ShaderManager::setUniform(const std::string& name, float value) {
    GLint loc = glGetUniformLocation(m_program, name.c_str());
    if (loc >= 0) glUniform1f(loc, value);
}
void ShaderManager::setUniform(const std::string& name, int value) {
    GLint loc = glGetUniformLocation(m_program, name.c_str());
    if (loc >= 0) glUniform1i(loc, value);
}
void ShaderManager::setUniform(const std::string& name, float v0, float v1) {
    GLint loc = glGetUniformLocation(m_program, name.c_str());
    if (loc >= 0) glUniform2f(loc, v0, v1);
}
void ShaderManager::setUniform(const std::string& name, float v0, float v1, float v2, float v3) {
    GLint loc = glGetUniformLocation(m_program, name.c_str());
    if (loc >= 0) glUniform4f(loc, v0, v1, v2, v3);
}