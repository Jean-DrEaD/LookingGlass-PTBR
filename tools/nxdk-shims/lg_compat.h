#pragma once
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

