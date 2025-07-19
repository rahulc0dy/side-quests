cmake_minimum_required(VERSION 3.31)
project(ShaderTester LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)

file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(${CMAKE_PROJECT_NAME}
        ${SOURCE_FILES}
        external/glad/glad.c
)

add_subdirectory(external/glfw)
add_subdirectory(external/glm)
add_subdirectory(external/imgui)
add_subdirectory(external/tinyfiledialogs)
add_subdirectory(external/glad)

find_package(OpenGL REQUIRED)

target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include
)

target_link_libraries(${CMAKE_PROJECT_NAME}
        OpenGL::GL
        glad
        glfw
        glm::glm
        imgui
        tinyfiledialogs
)

if(UNIX AND NOT APPLE)
    target_link_libraries(shader_tester dl pthread)
endif()


