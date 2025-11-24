use std::ffi::CStr;
use std::os::raw::c_char;

slint::include_modules!();

// 声明 C++ 函数
extern "C" {
    fn get_hello_cpp() -> *const c_char;
    fn get_hello_taskflow() -> *const c_char;
    fn get_hello_kokkos() -> *const c_char;
}

fn main() -> Result<(), slint::PlatformError> {
    // 调用 C++ 函数获取 "hello c++"
    let hello_cpp = unsafe {
        let c_str = get_hello_cpp();
        CStr::from_ptr(c_str)
            .to_str()
            .unwrap_or("hello c++")
    };
    
    // 调用 C++ 函数获取 "hello taskflow"（使用 TaskFlow 库）
    let hello_taskflow = unsafe {
        let c_str = get_hello_taskflow();
        CStr::from_ptr(c_str)
            .to_str()
            .unwrap_or("hello taskflow")
    };
    
    // 调用 C++ 函数获取 "hello kokkos"（使用 Kokkos 库）
    let hello_kokkos = unsafe {
        let c_str = get_hello_kokkos();
        CStr::from_ptr(c_str)
            .to_str()
            .unwrap_or("hello kokkos")
    };
    
    let ui = AppWindow::new()?;
    
    // 设置 UI 中的文本
    ui.set_hello_cpp(hello_cpp.into());
    ui.set_hello_slint("hello slint".into());
    ui.set_hello_taskflow(hello_taskflow.into());
    ui.set_hello_kokkos(hello_kokkos.into());
    
    ui.run()
}

