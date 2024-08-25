
# include "my_world_cpp_host.h"

void test::example::my_interface::MyObject::Set(uint32_t v) {
    value = v;
}

uint32_t test::example::my_interface::MyObject::Get() {
    return value;
}

test::example::my_interface::MyObject::MyObject(WASMExecEnv* e, uint32_t v)
 : value(v) {
}
