#include "Controls.hpp"

Controls::Controls(GLFWwindow* window): m_window(nullptr), m_play(false) {
    m_window = window;
}

Controls::~Controls() {
    m_window = nullptr;
}
