#pragma once
/* LookingGlass: prologo nxdk */
#define LG_NXDK 1
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <errno.h>
/* LookingGlass: compat POSIX para nxdk */
#include <ctype.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#ifdef __cplusplus
extern "C" {
#endif
static inline int lg_strncasecmp(const char *a, const char *b, size_t n) {
  for (; n; --n, ++a, ++b) {
    int d = tolower((unsigned char)*a) - tolower((unsigned char)*b);
    if (d || !*a) return d;
  }
  return 0;
}
static inline int lg_strcasecmp(const char *a, const char *b) { return lg_strncasecmp(a, b, (size_t)-1); }
static inline struct tm *lg_localtime_r(const time_t *t, struct tm *r) { struct tm *p = localtime(t); if (!p) return 0; *r = *p; return r; }
static inline struct tm *lg_gmtime_r(const time_t *t, struct tm *r) { struct tm *p = gmtime(t); if (!p) return 0; *r = *p; return r; }
#ifdef __cplusplus
}
#endif
#define strcasecmp  lg_strcasecmp
#define strncasecmp lg_strncasecmp
#define localtime_r lg_localtime_r
#define gmtime_r    lg_gmtime_r


#ifdef __cplusplus
extern "C" {
#endif
static inline int lg_rmdir(const char *p) { return RemoveDirectoryA(p) ? 0 : -1; }
#ifdef __cplusplus
}
#endif
#define rmdir lg_rmdir
/* windows.h polui nomes usados pelo engine */
#undef near
#undef far
#undef min
#undef max

/* FAR/NEAR vazios (zlib); near/far minusculos continuam livres */
#undef FAR
#define FAR
#undef NEAR
#define NEAR
