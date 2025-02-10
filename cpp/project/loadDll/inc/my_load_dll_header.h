#ifndef MY_LOAD_DLL_HEADER
#define MY_LOAD_DLL_HEADER

#define CALL __stdcall

#ifdef __cplusplus
extern "C" {
#endif

    void* CALL load_dll(const char*);
    void* CALL get_func(void*,const char*);
    void CALL free_dll(void*);

#ifdef __cplusplus
}
#endif

#endif
