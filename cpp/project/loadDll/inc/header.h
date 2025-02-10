#ifndef MYDLL_H
#define MYDLL_H


#define CALL __stdcall
#ifdef __cplusplus
extern "C" {
#endif
    double CALL add(double, double);
    double CALL mul(double, double);
#ifdef __cplusplus
}
#endif
#endif

