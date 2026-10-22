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
  static char d[260]; size_t n; strncpy(d, p, 259); d[259] = 0;
  n = strlen(d); while (n > 0 && d[n-1] != '\\' && d[n-1] != '/') n--;
  if (n > 0) n--; d[n] = 0; return d;
}

/* ---- arquivos ---- */
FILE *Sys_FOpen(const char *p, const char *m) { return fopen(p, m); }
FILE *Sys_Mkfifo(const char *p) { (void)p; return NULL; }
qboolean Sys_Mkdir(const char *p) { return CreateDirectoryA(p, NULL) || GetLastError() == ERROR_ALREADY_EXISTS; }
/* TODO(lote 15): listar via FindFirstFileA (necessario para .pk3) */
char **Sys_ListFiles(const char *dir, const char *ext, const char *filter, int *num, qboolean subs) {
  char **l = (char **)calloc(1, sizeof(char *)); (void)dir; (void)ext; (void)filter; (void)subs;
  *num = 0; return l;
}
void Sys_FreeFileList(char **l) { int i; if (!l) return; for (i = 0; l[i]; i++) free(l[i]); free(l); }

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
double atof(const char *s) { return strtod(s, NULL); }

/* ---- runtime C++ EH (STUB: throw aborta) ---- */
void __std_terminate(void) { abort(); }
int  __CxxFrameHandler3(void *a, void *b, void *c, void *d) { (void)a; (void)b; (void)c; (void)d; return 1; /* ContinueSearch */ }
void __stdcall _CxxThrowException(void *o, void *t) { (void)o; (void)t; OutputDebugStringA("LG: C++ throw sem runtime EH\n"); abort(); }
