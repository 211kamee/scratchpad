cmake_minimum_required(VERSION 4.4)
project(vulkan-garage VERSION 0.1.0 LANGUAGES C)

set(CMAKE_C_STANDARD 17)

find_package(Vulkan REQUIRED)

include(FetchContent)

FetchContent_Declare(
    glfw
    GIT_REPOSITORY https://github.com/glfw/glfw.git
    GIT_TAG 3.4
)

FetchContent_Declare(
    glm
    GIT_REPOSITORY https://github.com/g-truc/glm.git
    GIT_TAG 1.0.1

)

FetchContent_MakeAvailable(glfw glm)

add_executable(vulkan-garage src/main.c)

target_link_libraries(vulkan-garage
    Vulkan::Vulkan
    glm
    glfw
)
