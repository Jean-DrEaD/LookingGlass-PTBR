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
void Sys_PlatformInit(void) { XVideoSetMode(640, 480, 32, REFRESH_DEFAULT); debugPrint("LG: OpenMoHAA Xbox boot\n"); }
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

/* ---- lote 15p: probe/mount D: ---- */
#include <nxdk/mount.h>
static void lg_probe_d(void) {
  static int done = 0; WIN32_FIND_DATAA fd; HANDLE h; int n = 0;
  if (done) return; done = 1;
  if (!nxIsDriveMounted('E')) debugPrint("LG: mount E->HD P1 = %d\n", nxMountDrive('E', "\\Device\\Harddisk0\\Partition1\\"));
  CreateDirectoryA("E:\\LG", NULL); CreateDirectoryA("E:\\LG\\omohaa", NULL);
  debugPrint("LG: attr E:\\LG\\omohaa = %lx\n", (unsigned long)GetFileAttributesA("E:\\LG\\omohaa"));
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
char *Sys_DefaultHomePath(void) { return "E:\\LG\\omohaa"; }
int Sys_Mkdir(const char *p) {
  char b[512]; char *w = b; const char *r = p; DWORD a;
  if (!p) return 0;
  while (*r && w < b + sizeof(b) - 1) { char ch = (*r == '/') ? '\\' : *r; r++; if (ch == '\\' && w > b && w[-1] == '\\') continue; *w++ = ch; }
  *w = 0;
  if (CreateDirectoryA(b, NULL)) return 1;
  a = GetFileAttributesA(b);
  return a != INVALID_FILE_ATTRIBUTES && (a & FILE_ATTRIBUTE_DIRECTORY);
}