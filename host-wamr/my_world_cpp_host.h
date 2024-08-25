// Generated by `wit-bindgen` 0.3.0. DO NOT EDIT!
#ifndef __CPP_HOST_BINDINGS_MY_WORLD_H
#define __CPP_HOST_BINDINGS_MY_WORLD_H
struct WASMExecEnv; // WAMR execution environment
#include <cassert>
#include <cstdint>
#include <map>
#include <utility>
#include <wit-host.h>
/* User class definition file, autogenerated once, then user modified
 * Updated versions of this file are generated into MyObject.template.
 */
namespace test {
namespace example {
namespace my_interface {
class MyObject : public wit::ResourceImportBase<MyObject> {
  int32_t value;

public:
  static void Dtor(MyObject *self) { delete self; }
  MyObject(WASMExecEnv *exec_env, uint32_t a);
  static Owned New(WASMExecEnv *exec_env, uint32_t a) {
    return Owned(new MyObject(exec_env, a));
  }
  void Set(uint32_t v);
  uint32_t Get();
};

// export_interface Interface(Id { idx: 0 })
} // namespace my_interface
} // namespace example
} // namespace test
namespace exports {
namespace test {
namespace example {
namespace my_interface {
class MyObject : public wit::ResourceExportBase {
  wasm_exec_env_t exec_env;

public:
  ~MyObject();
  MyObject(WASMExecEnv *exec_env, uint32_t a);
  void Set(uint32_t v) const;
  uint32_t Get() const;
  MyObject(wasm_exec_env_t _exec_env, wit::ResourceExportBase &&);
  MyObject(MyObject &&) = default;
  MyObject &operator=(MyObject &&) = default;
};

} // namespace my_interface
} // namespace example
} // namespace test
} // namespace exports
extern "C" void register_my_world();

#endif
