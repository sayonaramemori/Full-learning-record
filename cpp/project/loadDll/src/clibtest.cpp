#include "my_load_dll_header.h"
#include <Windows.h>

#ifdef __cplusplus
extern "C" {
#endif

void* CALL load_dll(const char* path) {
    HMODULE hModule = LoadLibraryA(path);  // Use LoadLibraryA for UTF-8 path strings
    if (hModule == NULL) {
        return NULL;
    }
    return (void*)hModule;
}
void* CALL get_func(void* hModule, const char* func) {
    HMODULE module_ptr = (HMODULE)hModule;
    FARPROC pFunc = GetProcAddress(module_ptr, func);  // Get function pointer by name
    return (void*)pFunc;
}
void CALL free_dll(void* hModule) {
    HMODULE module_ptr = (HMODULE)hModule;
    if (module_ptr != NULL) {
        FreeLibrary(module_ptr);
    }
}

#ifdef __cplusplus
}
#endif

