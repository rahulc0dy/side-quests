project(imgui)

file(GLOB IMGUI_SRC
        "imgui.cpp"
        "imgui_draw.cpp"
        "imgui_widgets.cpp"
        "imgui_tables.cpp"
        "backends/imgui_impl_glfw.cpp"
        "backends/imgui_impl_opengl3.cpp"
)

add_library(imgui STATIC ${IMGUI_SRC})

target_include_directories(imgui PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/backends
        ${CMAKE_SOURCE_DIR}/external/glfw/include
)