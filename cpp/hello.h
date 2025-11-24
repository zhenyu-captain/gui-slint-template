#ifndef HELLO_H
#define HELLO_H

#ifdef __cplusplus
extern "C" {
#endif

// 返回 "hello c++" 字符串
const char* get_hello_cpp();

// 返回 "hello taskflow" 字符串（使用 TaskFlow 库）
const char* get_hello_taskflow();

// 返回 "hello kokkos" 字符串（使用 Kokkos 库）
const char* get_hello_kokkos();

#ifdef __cplusplus
}
#endif

#endif // HELLO_H

