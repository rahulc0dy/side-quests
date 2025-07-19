#pragma once
#include "GLFW/glfw3.h"

class Controls {
public:
    Controls(GLFWwindow*);
    ~Controls();

private:
    GLFWwindow *m_window;
    bool m_play;


};
