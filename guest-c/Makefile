CC = /opt/wasi-sdk/bin/clang
CFLAGS = -O3 --target=wasm32-wasi 

guest-c.wasm: main.c my_world.c my_world_component_type.o
	$(CC) -o $@ $(CFLAGS) $^
