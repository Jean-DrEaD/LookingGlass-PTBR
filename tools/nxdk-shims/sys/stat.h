#pragma once
/* LookingGlass: shim nxdk sys/stat.h */
#include <sys/types.h>
#include <time.h>
#include <windows.h>
#define S_IFMT  0xF000
#define S_IFDIR 0x4000
#define S_IFREG 0x8000
#define S_ISDIR(m) (((m) & S_IFMT) == S_IFDIR)
#define S_ISREG(m) (((m) & S_IFMT) == S_IFREG)
#define _S_IFDIR S_IFDIR
#define _S_IFREG S_IFREG
#ifdef __cplusplus
extern "C" {
#endif
struct stat { mode_t st_mode; off_t st_size; time_t st_mtime; time_t st_atime; time_t st_ctime; };
static inline time_t lg_ft2t(FILETIME f) {
  unsigned long long v = ((unsigned long long)f.dwHighDateTime << 32) | f.dwLowDateTime;
  return (time_t)((v - 116444736000000000ULL) / 10000000ULL);
}
static inline int stat(const char *p, struct stat *s) {
  WIN32_FILE_ATTRIBUTE_DATA d;
  if (!GetFileAttributesExA(p, GetFileExInfoStandard, &d)) return -1;
  s->st_mode  = (d.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) ? S_IFDIR : S_IFREG;
  s->st_size  = (off_t)d.nFileSizeLow;
  s->st_mtime = lg_ft2t(d.ftLastWriteTime);
  s->st_atime = lg_ft2t(d.ftLastAccessTime);
  s->st_ctime = lg_ft2t(d.ftCreationTime);
  return 0;
}
static inline int mkdir(const char *p, mode_t m) { (void)m; return CreateDirectoryA(p, NULL) ? 0 : -1; }
#ifdef __cplusplus
}
#endif
#define _stat stat
