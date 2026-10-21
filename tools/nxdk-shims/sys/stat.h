#pragma once
/* LookingGlass: shim nxdk sys/stat.h */
#include <stat.h>
#include <windows.h>
static inline int lg_mkdir(const char *p) { return CreateDirectoryA(p, NULL) ? 0 : -1; }
static inline int lg_rmdir(const char *p) { return RemoveDirectoryA(p) ? 0 : -1; }
#ifndef mkdir
#define mkdir(p, ...) lg_mkdir(p)
#endif
#ifndef rmdir
#define rmdir(p) lg_rmdir(p)
#endif
