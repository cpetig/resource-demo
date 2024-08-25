#include <stdio.h>
#include <malloc.h>
#include "my_world.h"

struct exports_test_example_my_interface_my_object_t {
    uint32_t value;
    exports_test_example_my_interface_own_my_object_t id;
};

exports_test_example_my_interface_own_my_object_t exports_test_example_my_interface_constructor_my_object(uint32_t a) {
    struct exports_test_example_my_interface_my_object_t* result = (struct exports_test_example_my_interface_my_object_t*)malloc(sizeof(struct exports_test_example_my_interface_my_object_t));
    result->value = a;
    result->id = exports_test_example_my_interface_my_object_new(result);
    return result->id;
}

void exports_test_example_my_interface_method_my_object_set(exports_test_example_my_interface_borrow_my_object_t self, uint32_t v) {
    self->value = v;
}

uint32_t exports_test_example_my_interface_method_my_object_get(exports_test_example_my_interface_borrow_my_object_t self) {
    return self->value;
}

void exports_test_example_my_interface_my_object_destructor(exports_test_example_my_interface_my_object_t *rep) {
    printf("Guest object is %d at drop\n", rep->value);
    exports_test_example_my_interface_my_object_drop_own(rep->id);
    free(rep);
}

int main() {
    test_example_my_interface_own_my_object_t o = test_example_my_interface_constructor_my_object(42);
    test_example_my_interface_borrow_my_object_t o2 = test_example_my_interface_borrow_my_object(o);
    test_example_my_interface_method_my_object_set(o2, 17);
    test_example_my_interface_method_my_object_set(o2, test_example_my_interface_method_my_object_get(o2)*2);
    printf("Object is %d\n", test_example_my_interface_method_my_object_get(o2));
    test_example_my_interface_my_object_drop_own(o);
    return 0;
}
