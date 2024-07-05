/*
 * Copyright (C) 2019 Intel Corporation.  All rights reserved.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include <stdlib.h>
#include <stdio.h>
#include "wasm_c_api.h"
#include "wasm_export.h"
#include <assert.h>

extern "C" void register_my_world();

static WASMFunctionInstanceCommon *cabi_alloc_ptr;

// returns address in guest memory
size_t guest_alloc(WASMExecEnv* exec_env, unsigned size) {
    const size_t alignment=1;

    wasm_val_t results[1] = {WASM_INIT_VAL};
    wasm_val_t arguments[4] = {WASM_I32_VAL(0), WASM_I32_VAL(0), WASM_I32_VAL((int32_t)alignment), WASM_I32_VAL((int32_t)size)};

    if (!wasm_runtime_call_wasm_a(exec_env, cabi_alloc_ptr, 1, results, 4, arguments))
    {
        const char *exception;
        if ((exception = wasm_runtime_get_exception(wasm_runtime_get_module_inst(exec_env))))
        {
            printf("Exception: %s\n", exception);
        }
        return 0;
    }
    return results[0].of.i32;
}

static char *
read_file_to_buffer(const char *filename, uint32_t *ret_size)
{
    FILE *file = fopen(filename, "rb");
    char *binary;
    int ret;
    long file_size;
    if (!file)
        return nullptr;
    ret = fseek(file, 0L, SEEK_END);
    if (ret == -1)
    {
        goto close_file;
    }
    file_size = ftell(file);
    if (file_size == -1)
    {
        goto close_file;
    }

    ret = fseek(file, 0L, SEEK_SET);
    if (ret == -1)
    {
        goto close_file;
    }

    binary = (char *)malloc(file_size);

    if (fread(binary, file_size, 1, file) != 1)
    {
        goto delete_binary;
    }

    fclose(file);
    *ret_size = file_size;
    return binary;

delete_binary:
    free(binary);

close_file:
    fclose(file);
    return nullptr;
}

// this is a modified version of https://github.com/bytecodealliance/wasm-micro-runtime/blob/main/samples/basic/src/main.c
int main()
{
    static char global_heap_buf[512 * 1024];
    char *buffer, error_buf[128];
    int opt;
    char const *wasm_path = "../../guest.wasm";

    wasm_module_t module = NULL;
    wasm_module_inst_t module_inst = NULL;
    wasm_exec_env_t exec_env = NULL;
    uint32_t buf_size, stack_size = 65536, heap_size = 4 * 65536;
    wasm_function_inst_t func = NULL;
    wasm_function_inst_t func2 = NULL;
    char *native_buffer = NULL;
    uint32_t wasm_buffer = 0;

    RuntimeInitArgs init_args;
    memset(&init_args, 0, sizeof(RuntimeInitArgs));

    init_args.mem_alloc_type = Alloc_With_System_Allocator;
    init_args.max_thread_num = 4;
    init_args.running_mode = Mode_Interp;

    if (!wasm_runtime_full_init(&init_args))
    {
        printf("Init runtime environment failed.\n");
        return -1;
    }

    register_my_world();

    buffer = read_file_to_buffer(wasm_path, &buf_size);

    if (!buffer)
    {
        printf("Open wasm app file [%s] failed.\n", wasm_path);
        goto fail;
    }

    module = wasm_runtime_load((uint8_t *)buffer, buf_size, error_buf, sizeof(error_buf));
    if (!module)
    {
        printf("Load wasm module failed. error: %s\n", error_buf);
        goto fail;
    }

    module_inst = wasm_runtime_instantiate(module, stack_size, heap_size,
                                           error_buf, sizeof(error_buf));

    if (!module_inst)
    {
        printf("Instantiate wasm module failed. error: %s\n", error_buf);
        goto fail;
    }

    exec_env = wasm_runtime_create_exec_env(module_inst, stack_size);
    if (!exec_env)
    {
        printf("Create wasm execution environment failed.\n");
        goto fail;
    }

    if (!(func = wasm_runtime_lookup_function(module_inst, "_start")))
    {
        printf("The _start wasm function is not found.\n");
        goto fail;
    }

    if (!(cabi_alloc_ptr = wasm_runtime_lookup_function(module_inst, "cabi_realloc")))
    {
        printf("The cabi_realloc wasm function is not found.\n");
        goto fail;
    }

    if (!wasm_runtime_call_wasm_a(exec_env, func, 0, nullptr, 0, nullptr))
    {
        printf("call wasm function _start failed. %s\n",
               wasm_runtime_get_exception(module_inst));
        goto fail;
    }

fail:
    if (exec_env)
        wasm_runtime_destroy_exec_env(exec_env);
    if (module_inst)
    {
        if (wasm_buffer)
            wasm_runtime_module_free(module_inst, wasm_buffer);
        wasm_runtime_deinstantiate(module_inst);
    }
    if (module)
        wasm_runtime_unload(module);
    if (buffer)
        free(buffer);
    wasm_runtime_destroy();
    return 0;
}
