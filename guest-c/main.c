#include <stdio.h>
#include "my_world.h"

int main() {
    test_example_my_interface_own_my_object_t o = test_example_my_interface_constructor_my_object(42);
    test_example_my_interface_borrow_my_object_t o2 = test_example_my_interface_borrow_my_object(o);
    test_example_my_interface_method_my_object_set(o2, 17);
    test_example_my_interface_method_my_object_set(o2, test_example_my_interface_method_my_object_get(o2)*2);
    printf("Object is %d\n", test_example_my_interface_method_my_object_get(o2));
    // my_world_my_object_drop_borrow(o2); - this would drop it twice
    test_example_my_interface_my_object_drop_own(o);
    return 0;
}
