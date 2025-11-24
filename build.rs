use std::path::Path;

fn main() {
    // 编译 Slint UI 文件
    slint_build::compile("ui/app.slint").unwrap();
    
    // 检查 TaskFlow 是否存在
    let taskflow_dir = Path::new("cpp/taskflow");
    let taskflow_header = Path::new("cpp/taskflow/taskflow/taskflow.hpp");
    
    if !taskflow_dir.exists() || !taskflow_header.exists() {
        println!("cargo:warning=TaskFlow 未找到！");
        println!("cargo:warning=请运行以下命令初始化 git submodule:");
        println!("cargo:warning=  git submodule update --init --recursive");
        println!("cargo:warning=或者手动下载 TaskFlow 到 cpp/taskflow 目录");
        panic!("TaskFlow 库未找到，请先初始化 git submodule");
    }
    
    // 检查 Kokkos 是否存在（使用包装或完整库）
    let kokkos_dir = Path::new("cpp/kokkos");
    let kokkos_header = Path::new("cpp/kokkos/Kokkos_Core.hpp");
    let kokkos_header_alt = Path::new("cpp/kokkos/core/src/Kokkos_Core.hpp");
    
    if !kokkos_dir.exists() || (!kokkos_header.exists() && !kokkos_header_alt.exists()) {
        println!("cargo:warning=Kokkos 未找到，使用最小化包装");
        // 继续使用包装版本
    }
    
    // 使用 cc crate 编译 C++ 代码
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .file("cpp/hello.cpp")
        .include("cpp")
        .include("cpp/taskflow");
    
    // 添加 Kokkos 包含路径
    if kokkos_header.exists() {
        build.include("cpp/kokkos");
    } else if kokkos_header_alt.exists() {
        build.include("cpp/kokkos/core/src");
    } else {
        build.include("cpp/kokkos");
    }
    
    build.compile("hello");
    
    // 告诉 Cargo 如果 C++ 文件改变，重新运行构建脚本
    println!("cargo:rerun-if-changed=cpp/hello.cpp");
    println!("cargo:rerun-if-changed=cpp/hello.h");
    println!("cargo:rerun-if-changed=cpp/taskflow");
    println!("cargo:rerun-if-changed=cpp/kokkos");
}

