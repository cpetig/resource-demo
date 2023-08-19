#include <iostream>
#include "my_world_cpp.h"

int main() {
    test::example::my_interface::MyObject o = test::example::my_interface::MyObject(42);
    o.Set(17);
    o.Set(o.Get()*2);
    std::cout << "Object is " << o.Get() << std::endl;
    return 0;
}
