#include "my_world_cpp_host.h"
#include <vector>
#include <iostream>

struct test::example::my_interface::MyObject::pImpl {
    int value;

    pImpl(int v) : value(v) {}
};

test::example::my_interface::MyObject::MyObject(uint32_t a) : p_impl(new pImpl(a))
{
    
}

uint32_t test::example::my_interface::MyObject::Get() {
    return p_impl->value;
}

void test::example::my_interface::MyObject::Set(uint32_t v) {
    p_impl->value = v;
}

test::example::my_interface::MyObject::~MyObject() {
    std::cout << "drop with value " << p_impl->value << std::endl;
    delete p_impl;
}

static std::vector<my_world::ResourceBase*> resources;

my_world::ResourceBase::ResourceBase() : id(resources.size()) {
    resources.push_back(this);
}

my_world::ResourceBase::~ResourceBase() {
    resources.at(id) = nullptr;
}

my_world::ResourceBase* my_world::ResourceBase::lookup_resource(int32_t id) {
    return resources.at(id);
}
