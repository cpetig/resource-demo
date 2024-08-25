CXX = /opt/wasi-sdk/bin/clang++
CFLAGS = -O3 --target=wasm32-wasi -std=c++17 -fno-exceptions -I.

guest-cpp.wasm: main.cpp my_world.cpp my_world_component_type.o
	$(CXX) -o $@ $(CFLAGS) $^
