cmake_minimum_required(VERSION 3.14)
project(hello_cpp)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 使用 FetchContent 下载 TaskFlow
include(FetchContent)

FetchContent_Declare(
    taskflow
    GIT_REPOSITORY https://github.com/taskflow/taskflow.git
    GIT_TAG v3.8.0
)

FetchContent_MakeAvailable(taskflow)

# 添加 C++ 源文件
add_library(hello STATIC
    hello.cpp
)

# 链接 TaskFlow（header-only，只需要包含目录）
target_include_directories(hello PUBLIC
    ${taskflow_SOURCE_DIR}/taskflow
)

# 设置输出目录
set_target_properties(hello PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

