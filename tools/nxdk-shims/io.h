#pragma once
/* LookingGlass: shim nxdk io.h */
#include <sys/types.h>
#include <fcntl.h>
#include <windows.h>
#ifdef __cplusplus
extern "C" {
#endif
int open(const char *path, int flags, ...);
int close(int fd);
int read(int fd, void *buf, unsigned int n);
int write(int fd, const void *buf, unsigned int n);
long lseek(int fd, long off, int whence);
static inline int _access(const char *p, int m) { (void)m; return GetFileAttributesA(p) == INVALID_FILE_ATTRIBUTES ? -1 : 0; }
static inline int _mkdir(const char *p) { return CreateDirectoryA(p, NULL) ? 0 : -1; }
#ifdef __cplusplus
}
#endif
#define _open  open
#define _close close
#define _read  read
#define _write write
#define _lseek lseek
#define access _access
