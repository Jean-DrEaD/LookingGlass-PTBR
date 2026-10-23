/* LookingGlass: camada de sistema Xbox (nxdk) para OpenMoHAA */
#include <windows.h>
#include <hal/debug.h>
#include <hal/video.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef int qboolean;
static char lg_home[] = "D:\\";

/* ---- tempo / threads ---- */
int  Sys_Milliseconds(void) { static DWORD b = 0; if (!b) b = GetTickCount(); return (int)(GetTickCount() - b); }
void Sys_Sleep(int ms) { Sleep(ms < 0 ? 0 : (DWORD)ms); }
void SetNormalThreadPriority(void) {}
void SetBelowNormalThreadPriority(void) {}

/* ---- caminhos ---- */
char *Sys_DefaultHomeConfigPath(void) { return lg_home; }
char *Sys_DefaultHomeDataPath(void)   { return lg_home; }
char *Sys_DefaultHomeStatePath(void)  { return lg_home; }
char *Sys_SteamPath(void)          { return ""; }
char *Sys_GogPath(void)            { return ""; }
char *Sys_MicrosoftStorePath(void) { return ""; }
char *Sys_Cwd(void)                { return lg_home; }
char *Sys_GetCurrentUser(void)     { return "player"; }
const char *Sys_Dirname(char *p) {
  static char d[260]; size_t n; if (!p) { d[0] = 0; debugPrint("LG: Sys_Dirname(NULL)\n"); return d; } strncpy(d, p, 259); d[259] = 0;
  n = strlen(d); while (n > 0 && d[n-1] != '\\' && d[n-1] != '/') n--;
  if (n > 0) n--; d[n] = 0; return d;
}

/* ---- arquivos ---- */
FILE *Sys_FOpen(const char *p, const char *m) { return fopen(p, m); }
FILE *Sys_Mkfifo(const char *p) { (void)p; return NULL; }
/* TODO(lote 15): listar via FindFirstFileA (necessario para .pk3) */

/* ---- processo / plataforma ---- */
qboolean Sys_RandomBytes(unsigned char *s, int n) { int i; for (i = 0; i < n; i++) s[i] = (unsigned char)(GetTickCount() * 2654435761u >> (i & 24)); return 0; }
/* LG 15w: montar drives antes do FS */
#include <nxdk/mount.h>
static void lg_mount_drives(void)
{
if (!nxIsDriveMounted('E')) debugPrint("LG: early mount E = %d\n", (int)nxMountDrive('E', "\\Device\\Harddisk0\\Partition1\\"));
if (!nxIsDriveMounted('D')) debugPrint("LG: early mount D = %d\n", (int)nxMountDrive('D', "\\Device\\CdRom0\\"));
}
void Sys_PlatformInit(void) {
	lg_mount_drives(); XVideoSetMode(640, 480, 32, REFRESH_DEFAULT); debugPrint("LG: OpenMoHAA Xbox boot\n"); }
void Sys_PlatformExit(void) {}
void Sys_SetEnv(const char *n, const char *v) { (void)n; (void)v; }
int  Sys_PID(void) { return 1; }
qboolean Sys_PIDIsRunning(int pid) { (void)pid; return 0; }
qboolean Sys_DllExtension(const char *n) { (void)n; return 0; }
qboolean Sys_OpenFolderInPlatformFileManager(const char *p) { (void)p; return 0; }
qboolean Sys_SetMaxFileLimit(void) { return 1; }
void Sys_ErrorDialog(const char *e) { OutputDebugStringA(e); debugPrint("ERRO: %s\n", e); }
void Sys_DebugPrint(const char *m) { OutputDebugStringA(m); debugPrint("%s", m); }
void Sys_PrintBackTrace(void) {}

/* ---- console ---- */
void  CON_Init(void) {}
void  CON_Shutdown(void) {}
char *CON_Input(void) { return NULL; }
void  CON_Print(const char *m) { OutputDebugStringA(m); debugPrint("%s", m); }

/* ---- libc ausente ---- */
/* ---- strtod proprio (nxdk strtod = assert(0)) ---- */
#include <ctype.h>
double lg_strtod(const char *s, char **end) {
  const char *p = s; double v = 0.0, f; int neg = 0, e = 0, en = 0, any = 0;
  while (isspace((unsigned char)*p)) p++;
  if (*p == '-' || *p == '+') neg = (*p++ == '-');
  while (*p >= '0' && *p <= '9') { v = v * 10.0 + (*p++ - '0'); any = 1; }
  if (*p == '.') { p++; f = 0.1; while (*p >= '0' && *p <= '9') { v += (*p++ - '0') * f; f *= 0.1; any = 1; } }
  if (any && (*p == 'e' || *p == 'E')) {
    const char *q = p + 1; if (*q == '-' || *q == '+') en = (*q++ == '-');
    if (*q >= '0' && *q <= '9') { while (*q >= '0' && *q <= '9') e = e * 10 + (*q++ - '0'); p = q;
      while (e--) v = en ? v / 10.0 : v * 10.0; }
  }
  if (!any) p = s;
  if (end) *end = (char *)p;
  return neg ? -v : v;
}
float lg_strtof(const char *s, char **end) { return (float)lg_strtod(s, end); }
double atof(const char *s) { return lg_strtod(s, NULL); }

/* ---- runtime C++ EH (STUB: throw aborta) ---- */
void __std_terminate(void) { debugPrint("LG: std::terminate\n"); for(;;) Sleep(1000); }
int  __CxxFrameHandler3(void *a, void *b, void *c, void *d) { (void)a; (void)b; (void)c; (void)d; return 1; /* ContinueSearch */ }
void __stdcall _CxxThrowException(void *o, void *t) { (void)o; (void)t; debugPrint("LG: C++ throw sem runtime EH\n"); for(;;) Sleep(1000); }

/* ---- lote 15t: rede (lwIP) antes do NET_Init ---- */
#include <nxdk/net.h>
static void lg_net_init(void) {
  int r;
  debugPrint("LG: nxNetInit...\n");
  r = nxNetInit(NULL);
  debugPrint("LG: nxNetInit = %d\n", r);
}
/* ---- lote 15p: probe/mount D: ---- */
#include <nxdk/mount.h>
static void lg_probe_d(void) {
  static int done = 0; WIN32_FIND_DATAA fd; HANDLE h; int n = 0;
  if (done) return; done = 1;
  if (!nxIsDriveMounted('E')) debugPrint("LG: mount E->HD P1 = %d\n", nxMountDrive('E', "\\Device\\Harddisk0\\Partition1\\"));
  CreateDirectoryA("E:\\TDATA", NULL); CreateDirectoryA("E:\\TDATA\\4c470001", NULL); CreateDirectoryA("E:\\UDATA", NULL); CreateDirectoryA("E:\\UDATA\\4c470001", NULL);
  debugPrint("LG: attr E:\\TDATA\\4c470001 = %lx\n", (unsigned long)GetFileAttributesA("E:\\TDATA\\4c470001"));
  debugPrint("LG: D mounted=%d\n", nxIsDriveMounted('D'));
  if (!nxIsDriveMounted('D')) debugPrint("LG: mount D->CdRom0 = %d\n", nxMountDrive('D', "\\Device\\CdRom0\\"));
  debugPrint("LG: attr D:\\default.xbe = %lx\n", (unsigned long)GetFileAttributesA("D:\\default.xbe"));
  h = FindFirstFileA("D:\\*", &fd);
  debugPrint("LG: D:\\* ok=%d err=%lu\n", h != INVALID_HANDLE_VALUE, h == INVALID_HANDLE_VALUE ? (unsigned long)GetLastError() : 0UL);
  if (h != INVALID_HANDLE_VALUE) { do { if (n++ < 6) debugPrint("LG:  %s\n", fd.cFileName); } while (FindNextFileA(h, &fd)); FindClose(h); }
}

/* ---- lote 15o: Sys_ListFiles real (FindFirstFileA) ---- */
#include <windows.h>
#include <string.h>
#include <stdlib.h>
#define LG_MAXFOUND 4096
char **Sys_ListFiles(const char *dir, const char *ext, char *filter, int *num, int wantsubs) {
  WIN32_FIND_DATAA fd; HANDLE h; char pat[300]; char *found[LG_MAXFOUND]; int n = 0, i;
  int dirsOnly = (ext && ext[0] == '/' && ext[1] == 0); size_t el = (ext && !dirsOnly) ? strlen(ext) : 0;
  (void)filter; *num = 0; lg_probe_d();
  _snprintf(pat, sizeof(pat), "%s\\*", dir); pat[sizeof(pat)-1] = 0;
  for (i = 0; pat[i]; i++) if (pat[i] == '/') pat[i] = '\\';
  { char *r = pat, *w = pat; while (*r) { if (*r == '\\' && r[1] == '\\') { r++; continue; } *w++ = *r++; } *w = 0; }
  h = FindFirstFileA(pat, &fd);
  debugPrint("LG: LF pat=%s ok=%d err=%lu\n", pat, h != INVALID_HANDLE_VALUE, h == INVALID_HANDLE_VALUE ? (unsigned long)GetLastError() : 0UL);
  if (h == INVALID_HANDLE_VALUE) return NULL;
  do {
    int isDir = (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) != 0; size_t nl = strlen(fd.cFileName);
    if (!strcmp(fd.cFileName, ".") || !strcmp(fd.cFileName, "..")) continue;
    if (dirsOnly && !isDir) continue;
    if (!dirsOnly && isDir && !wantsubs) continue;
    if (el && (nl < el || _stricmp(fd.cFileName + nl - el, ext))) continue;
    if (n < LG_MAXFOUND - 1) { found[n] = (char *)malloc(nl + 1); memcpy(found[n], fd.cFileName, nl + 1); n++; }
  } while (FindNextFileA(h, &fd));
  FindClose(h);
  if (!n) return NULL;
  { char **list = (char **)malloc((n + 1) * sizeof(char *));
    for (i = 0; i < n; i++) list[i] = found[i]; list[n] = NULL; *num = n;
    debugPrint("LG: ListFiles %s (%s) = %d\n", dir, ext ? ext : "", n); return list; }
}
void Sys_FreeFileList(char **list) { int i; if (!list) return; for (i = 0; list[i]; i++) free(list[i]); free(list); }

/* ---- lote 15q: lg_fopen ---- */
#undef fopen
FILE *fopen(const char *p, const char *m);
FILE *lg_fopen(const char *p, const char *m) {
  static int n = 0; char b[512]; char *w = b; const char *r = p; FILE *f;
  if (!p) return NULL;
  while (*r && w < b + sizeof(b) - 1) {
    char ch = (*r == '/') ? '\\' : *r; r++;
    if (ch == '\\' && w > b && w[-1] == '\\') continue;
    *w++ = ch;
  }
  *w = 0;
  f = fopen(b, m);
  if (n++ < 12) debugPrint("LG: fopen %s [%s] = %d\n", b, m, f != NULL);
  return f;
}
/* ---- lote 15r: sobrescreve os simbolos strtod/strtof/strtold do nxdk ---- */
#undef strtod
#undef strtof
#undef strtold
double lg_strtod(const char *s, char **e);
double strtod(const char *s, char **e) { return lg_strtod(s, e); }
float strtof(const char *s, char **e) { return (float)lg_strtod(s, e); }
long double strtold(const char *s, char **e) { return (long double)lg_strtod(s, e); }
/* ---- lote 15s: homepath no HD + Sys_Mkdir tolerante ---- */
char *Sys_DefaultHomePath(void) { return "E:\\TDATA\\4c470001"; }
int Sys_Mkdir(const char *p) {
  char b[512]; char *w = b; const char *r = p; DWORD a;
  if (!p) return 0;
  while (*r && w < b + sizeof(b) - 1) { char ch = (*r == '/') ? '\\' : *r; r++; if (ch == '\\' && w > b && w[-1] == '\\') continue; *w++ = ch; }
  *w = 0;
  if (CreateDirectoryA(b, NULL)) return 1;
  a = GetFileAttributesA(b);
  return a != INVALID_FILE_ATTRIBUTES && (a & FILE_ATTRIBUTE_DIRECTORY);
}

/* ---- lote 16z6b: lg_vsnprintf (PDCLib do nxdk nao formata %f/%e/%g) ---- */
#include <stdarg.h>
#include <stddef.h>
#include <string.h>
typedef struct { char *b; size_t n, k; } lg_out_t;
static void lg_pc(lg_out_t *o, char c) { if (o->k + 1 < o->n) o->b[o->k] = c; o->k++; }
static void lg_pad(lg_out_t *o, char c, int n) { while (n-- > 0) lg_pc(o, c); }
static void lg_emit(lg_out_t *o, const char *pre, const char *s, int len, int width, int left, int zero) {
  int pl = (int)strlen(pre), padn = width - pl - len, i;
  if (!left && !zero) lg_pad(o, ' ', padn);
  while (*pre) lg_pc(o, *pre++);
  if (!left && zero) lg_pad(o, '0', padn);
  for (i = 0; i < len; i++) lg_pc(o, s[i]);
  if (left) lg_pad(o, ' ', padn);
}
static int lg_utoa(char *buf, unsigned long long v, int base, int upper) {
  char t[32]; int n = 0, i; const char *dg = upper ? "0123456789ABCDEF" : "0123456789abcdef";
  do { t[n++] = dg[v % (unsigned)base]; v /= (unsigned)base; } while (v);
  for (i = 0; i < n; i++) buf[i] = t[n - 1 - i];
  return n;
}
static void lg_int(lg_out_t *o, unsigned long long u, int base, int upper, const char *pre, int prec, int width, int left, int zero) {
  char t[32], d[96]; int n, k = 0, i;
  if (prec >= 0) zero = 0; else prec = 1;
  n = (prec == 0 && u == 0) ? 0 : lg_utoa(t, u, base, upper);
  for (i = n; i < prec && k < 60; i++) d[k++] = '0';
  memcpy(d + k, t, n); k += n;
  lg_emit(o, pre, d, k, width, left, zero);
}
static int lg_ftoa_e(char *buf, double v, int prec, int upper);
static int lg_ftoa_f(char *buf, double v, int prec) {
  unsigned long long p10 = 1, ip, fd; double fr; int i, n;
  if (prec > 17) prec = 17;
  if (v >= 1.8e19) return lg_ftoa_e(buf, v, prec, 0);
  for (i = 0; i < prec; i++) p10 *= 10;
  ip = (unsigned long long)v;
  fr = v - (double)ip;
  fd = (unsigned long long)(fr * (double)p10 + 0.5);
  if (fd >= p10) { ip++; fd -= p10; }
  n = lg_utoa(buf, ip, 10, 0);
  if (prec > 0) {
    char t[24]; int m = lg_utoa(t, fd, 10, 0);
    buf[n++] = '.';
    for (i = m; i < prec; i++) buf[n++] = '0';
    memcpy(buf + n, t, m); n += m;
  }
  return n;
}
static int lg_ftoa_e(char *buf, double v, int prec, int upper) {
  int ex = 0, n;
  if (prec > 17) prec = 17;
  if (v != 0) { while (v >= 10) { v /= 10; ex++; } while (v < 1) { v *= 10; ex--; } }
  n = lg_ftoa_f(buf, v, prec);
  if (buf[0] == '1' && buf[1] == '0') { v /= 10; ex++; n = lg_ftoa_f(buf, v, prec); }
  buf[n++] = upper ? 'E' : 'e';
  buf[n++] = ex < 0 ? '-' : '+';
  if (ex < 0) ex = -ex;
  if (ex < 10) buf[n++] = '0';
  n += lg_utoa(buf + n, (unsigned long long)ex, 10, 0);
  return n;
}
static int lg_ftoa_g(char *buf, double v, int prec, int alt, int upper) {
  int P = prec < 0 ? 6 : (prec == 0 ? 1 : prec), n, X = 0, i, j, m;
  char t[64];
  m = lg_ftoa_e(t, v, P - 1, 0);
  for (i = 0; i < m; i++) if (t[i] == 'e') { int s = t[i + 1] == '-' ? -1 : 1; for (j = i + 2; j < m; j++) X = X * 10 + (t[j] - '0'); X *= s; break; }
  if (X < P && X >= -4) n = lg_ftoa_f(buf, v, P - 1 - X);
  else n = lg_ftoa_e(buf, v, P - 1, upper);
  if (!alt) {
    int e = n, hasdot = 0, z;
    for (i = 0; i < n; i++) if (buf[i] == 'e' || buf[i] == 'E') { e = i; break; }
    for (i = 0; i < e; i++) if (buf[i] == '.') hasdot = 1;
    if (hasdot) {
      z = e; while (z > 0 && buf[z - 1] == '0') z--;
      if (z > 0 && buf[z - 1] == '.') z--;
      memmove(buf + z, buf + e, n - e); n -= e - z;
    }
  }
  return n;
}
int lg_vsnprintf(char *buf, size_t size, const char *fmt, va_list ap) {
  lg_out_t o; char tmp[400];
  o.b = buf; o.n = size; o.k = 0;
  for (; *fmt; fmt++) {
    int left = 0, plus = 0, space = 0, alt = 0, zero = 0, width = 0, prec = -1, lng = 0;
    const char *p; char cv; char pre[4] = { 0, 0, 0, 0 };
    if (*fmt != '%') { lg_pc(&o, *fmt); continue; }
    p = fmt + 1;
    for (;; p++) { if (*p == '-') left = 1; else if (*p == '+') plus = 1; else if (*p == ' ') space = 1; else if (*p == '#') alt = 1; else if (*p == '0') zero = 1; else break; }
    if (*p == '*') { width = va_arg(ap, int); if (width < 0) { left = 1; width = -width; } p++; }
    else while (*p >= '0' && *p <= '9') width = width * 10 + (*p++ - '0');
    if (*p == '.') { p++; prec = 0; if (*p == '*') { prec = va_arg(ap, int); p++; } else while (*p >= '0' && *p <= '9') prec = prec * 10 + (*p++ - '0'); }
    for (;; p++) {
      if (*p == 'l') lng++;
      else if (*p == 'h' || *p == 'L' || *p == 'z' || *p == 't') ;
      else if (*p == 'j' || *p == 'q') lng = 2;
      else if (*p == 'I' && p[1] == '6' && p[2] == '4') { lng = 2; p += 2; }
      else if (*p == 'I' && p[1] == '3' && p[2] == '2') { p += 2; }
      else break;
    }
    cv = *p; fmt = p;
    if (!cv) break;
    if (left) zero = 0;
    switch (cv) {
    case 'd': case 'i': {
      long long v = lng >= 2 ? va_arg(ap, long long) : (long long)va_arg(ap, int);
      unsigned long long u = v < 0 ? 0ULL - (unsigned long long)v : (unsigned long long)v;
      if (v < 0) pre[0] = '-'; else if (plus) pre[0] = '+'; else if (space) pre[0] = ' ';
      lg_int(&o, u, 10, 0, pre, prec, width, left, zero); break; }
    case 'u': case 'x': case 'X': case 'o': {
      unsigned long long u = lng >= 2 ? va_arg(ap, unsigned long long) : (unsigned long long)va_arg(ap, unsigned int);
      int base = cv == 'o' ? 8 : (cv == 'u' ? 10 : 16);
      if (alt && u && base == 16) { pre[0] = '0'; pre[1] = cv; }
      if (alt && base == 8) pre[0] = '0';
      lg_int(&o, u, base, cv == 'X', pre, prec, width, left, zero); break; }
    case 'p': {
      unsigned long long u = (unsigned long long)(size_t)va_arg(ap, void *);
      lg_int(&o, u, 16, 0, "0x", 8, width, left, 0); break; }
    case 'c': tmp[0] = (char)va_arg(ap, int); lg_emit(&o, "", tmp, 1, width, left, 0); break;
    case 's': {
      const char *s = va_arg(ap, const char *); int len = 0;
      if (!s) s = "(null)";
      while (s[len] && (prec < 0 || len < prec)) len++;
      lg_emit(&o, "", s, len, width, left, 0); break; }
    case 'f': case 'F': case 'e': case 'E': case 'g': case 'G': {
      double v = va_arg(ap, double); int len;
      if (v < 0) { pre[0] = '-'; v = -v; } else if (plus) pre[0] = '+'; else if (space) pre[0] = ' ';
      if (v != v) { lg_emit(&o, pre, "nan", 3, width, left, 0); break; }
      if (v > 1.7976931348623157e308) { lg_emit(&o, pre, "inf", 3, width, left, 0); break; }
      if (cv == 'f' || cv == 'F') len = lg_ftoa_f(tmp, v, prec < 0 ? 6 : prec);
      else if (cv == 'e' || cv == 'E') len = lg_ftoa_e(tmp, v, prec < 0 ? 6 : prec, cv == 'E');
      else len = lg_ftoa_g(tmp, v, prec, alt, cv == 'G');
      lg_emit(&o, pre, tmp, len, width, left, zero); break; }
    case 'n': { int *pn = va_arg(ap, int *); if (pn) *pn = (int)o.k; break; }
    case '%': lg_pc(&o, '%'); break;
    default: lg_pc(&o, '%'); lg_pc(&o, cv); break;
    }
  }
  if (size) buf[o.k < size ? o.k : size - 1] = '\0';
  return (int)o.k;
}

/* ---- lote 16z6d: sscanf/vsscanf proprios (PDCLib do nxdk nao le %f) ---- */
#include <stdarg.h>
#include <stddef.h>
#include <string.h>
double lg_strtod(const char *s, char **end);
static int lg_sp(int c) { return c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\v' || c == '\f'; }
static int lg_dig(int c, int base) {
  int v;
  if (c >= '0' && c <= '9') v = c - '0';
  else if (c >= 'a' && c <= 'z') v = c - 'a' + 10;
  else if (c >= 'A' && c <= 'Z') v = c - 'A' + 10;
  else return -1;
  return v < base ? v : -1;
}
int lg_vsscanf(const char *str, const char *fmt, va_list ap) {
  const char *s = str; int cnt = 0;
  for (; *fmt; fmt++) {
    int sup = 0, width = 0, lng = 0, hh = 0; char cv;
    if (lg_sp((unsigned char)*fmt)) { while (lg_sp((unsigned char)*s)) s++; continue; }
    if (*fmt != '%') { if (*s != *fmt) break; s++; continue; }
    fmt++;
    if (*fmt == '%') { while (lg_sp((unsigned char)*s)) s++; if (*s != '%') break; s++; continue; }
    if (*fmt == '*') { sup = 1; fmt++; }
    while (*fmt >= '0' && *fmt <= '9') width = width * 10 + (*fmt++ - '0');
    for (;; fmt++) {
      if (*fmt == 'l') lng++;
      else if (*fmt == 'h') hh++;
      else if (*fmt == 'L' || *fmt == 'q' || *fmt == 'j') lng = 2;
      else if (*fmt == 'z' || *fmt == 't') ;
      else break;
    }
    cv = *fmt; if (!cv) break;
    if (cv == 'n') { if (!sup) *va_arg(ap, int *) = (int)(s - str); continue; }
    if (cv != 'c' && cv != '[') while (lg_sp((unsigned char)*s)) s++;
    if (!*s) return cnt ? cnt : -1;
    if (cv == 'd' || cv == 'i' || cv == 'u' || cv == 'x' || cv == 'X' || cv == 'o' || cv == 'p') {
      int base = (cv == 'd' || cv == 'u') ? 10 : (cv == 'o' ? 8 : (cv == 'i' ? 0 : 16));
      int neg = 0, w = width ? width : (1 << 30); unsigned long long v = 0; const char *st;
      if ((*s == '-' || *s == '+') && w > 0) { neg = *s == '-'; s++; w--; }
      if ((base == 0 || base == 16) && w >= 2 && s[0] == '0' && (s[1] == 'x' || s[1] == 'X') && lg_dig((unsigned char)s[2], 16) >= 0) { s += 2; w -= 2; base = 16; }
      else if (base == 0) base = (*s == '0') ? 8 : 10;
      st = s;
      while (w > 0 && lg_dig((unsigned char)*s, base) >= 0) { v = v * (unsigned)base + (unsigned)lg_dig((unsigned char)*s, base); s++; w--; }
      if (s == st) break;
      if (neg) v = 0ULL - v;
      if (!sup) {
        if (cv == 'p') *va_arg(ap, void **) = (void *)(size_t)v;
        else if (lng >= 2) *va_arg(ap, long long *) = (long long)v;
        else if (lng == 1) *va_arg(ap, long *) = (long)v;
        else if (hh >= 2) *va_arg(ap, char *) = (char)v;
        else if (hh == 1) *va_arg(ap, short *) = (short)v;
        else *va_arg(ap, int *) = (int)v;
        cnt++;
      }
      continue;
    }
    if (cv == 'f' || cv == 'F' || cv == 'e' || cv == 'E' || cv == 'g' || cv == 'G' || cv == 'a') {
      char nb[128]; char *ep; double d; int w = (width && width < 127) ? width : 127, k;
      for (k = 0; k < w && s[k]; k++) nb[k] = s[k];
      nb[k] = 0;
      d = lg_strtod(nb, &ep);
      if (ep == nb) break;
      s += ep - nb;
      if (!sup) { if (lng) *va_arg(ap, double *) = d; else *va_arg(ap, float *) = (float)d; cnt++; }
      continue;
    }
    if (cv == 's') {
      char *o = sup ? 0 : va_arg(ap, char *); int w = width ? width : (1 << 30);
      while (*s && !lg_sp((unsigned char)*s) && w > 0) { if (o) *o++ = *s; s++; w--; }
      if (o) { *o = 0; cnt++; }
      continue;
    }
    if (cv == 'c') {
      char *o = sup ? 0 : va_arg(ap, char *); int w = width ? width : 1, k;
      for (k = 0; k < w && *s; k++) { if (o) o[k] = *s; s++; }
      if (k == 0) break;
      if (o) cnt++;
      continue;
    }
    if (cv == '[') {
      char set[256]; int inv = 0, k, w = width ? width : (1 << 30); char *o; const char *st = s;
      memset(set, 0, sizeof set); fmt++;
      if (*fmt == '^') { inv = 1; fmt++; }
      if (*fmt == ']') { set[(unsigned char)']'] = 1; fmt++; }
      while (*fmt && *fmt != ']') {
        if (fmt[1] == '-' && fmt[2] && fmt[2] != ']') { for (k = (unsigned char)fmt[0]; k <= (unsigned char)fmt[2]; k++) set[k] = 1; fmt += 3; }
        else { set[(unsigned char)*fmt] = 1; fmt++; }
      }
      if (!*fmt) break;
      o = sup ? 0 : va_arg(ap, char *);
      while (*s && w > 0 && (set[(unsigned char)*s] ^ inv)) { if (o) *o++ = *s; s++; w--; }
      if (s == st) break;
      if (o) { *o = 0; cnt++; }
      continue;
    }
    break;
  }
  return cnt;
}
int vsscanf(const char *s, const char *f, va_list ap) { return lg_vsscanf(s, f, ap); }
int sscanf(const char *s, const char *f, ...) { va_list ap; int r; va_start(ap, f); r = lg_vsscanf(s, f, ap); va_end(ap); return r; }
