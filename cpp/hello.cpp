#include "hello.h"
#include <taskflow/taskflow.hpp>
#include <Kokkos_Core.hpp>
#include <string>
#include <memory>

// 使用静态变量存储字符串，确保在函数返回后仍然有效
static std::string taskflow_result;
static std::string kokkos_result;

extern "C" {
    const char* get_hello_cpp() {
        return "hello c++";
    }
    
    const char* get_hello_taskflow() {
        // 使用 TaskFlow 创建一个简单的任务流
        tf::Executor executor;
        tf::Taskflow taskflow;
        
        // 创建一个任务来生成 "hello taskflow" 字符串
        taskflow.emplace([&]() {
            taskflow_result = "hello taskflow";
        });
        
        // 执行任务流
        executor.run(taskflow).wait();
        
        return taskflow_result.c_str();
    }
    
    const char* get_hello_kokkos() {
        // 初始化 Kokkos（如果使用完整库）
        Kokkos::initialize();
        
        // 使用 Kokkos 创建一个简单的并行操作
        Kokkos::parallel_for("hello_kokkos", 1, [&](size_t i) {
            kokkos_result = "hello kokkos";
        });
        
        // 最终化 Kokkos
        Kokkos::finalize();
        
        return kokkos_result.c_str();
    }
}

