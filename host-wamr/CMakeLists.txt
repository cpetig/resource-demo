cmake_minimum_required(VERSION 3.14)
project(wasm_executor)

set(CMAKE_CXX_STANDARD 17)
if(APPLE)
  add_definitions(-DBH_PLATFORM_DARWIN)
endif()

set (WAMR_BUILD_PLATFORM "linux")
set (WAMR_BUILD_TARGET "X86_64")
set (WAMR_BUILD_INTERP 1)
set (WAMR_BUILD_FAST_INTERP 0)
set (WAMR_BUILD_JIT 0)
set (WAMR_BUILD_FAST_JIT 0)
set (WAMR_BUILD_AOT 0)
set (WAMR_BUILD_LIBC_BUILTIN 1)
set (WAMR_BUILD_LIBC_WASI 1)
set (WAMR_BUILD_SIMD 1)
set (WAMR_BUILD_LIB_WASI_THREADS 1)
set (WAMR_ROOT_DIR wasm-micro-runtime)

include (${WAMR_ROOT_DIR}/build-scripts/runtime_lib.cmake)
add_library(vmlib ${WAMR_RUNTIME_LIB_SOURCE})

add_executable(executor executor.cpp my_world_host.cpp implementation.cpp)
target_link_libraries (executor vmlib)
target_include_directories(executor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
