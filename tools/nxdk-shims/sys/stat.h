#pragma once
/* LookingGlass: shim nxdk sys/stat.h */
#if __has_include(<stat.h>)
#include <stat.h>
#else
#include <sys/types.h>
struct stat { unsigned int st_mode; long st_size; long st_mtime; };
#define S_IFDIR 0040000
#define S_IFREG 0100000
#define S_ISDIR(m) (((m) & 0170000) == S_IFDIR)
#define S_ISREG(m) (((m) & 0170000) == S_IFREG)
int stat(const char *path, struct stat *buf);
#endif
