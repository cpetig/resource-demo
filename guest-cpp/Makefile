CXX = /opt/wasi-sdk/bin/clang++
CFLAGS = -O3 --target=wasm32-wasi -std=c++17 -fno-exceptions

guest-cpp.wasm: main.cpp my_world.cpp
	$(CXX) -o $@ $(CFLAGS) $^
