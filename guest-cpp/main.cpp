#include <iostream>
#include "my_world_cpp.h"

//extern "C" void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size);

__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}


int main() {
    test::example::my_interface::MyObject o = test::example::my_interface::MyObject(42);
    o.Set(17);
    o.Set(o.Get()*2);
    std::cout << "Object is " << o.Get() << std::endl;
    return 0;
}
