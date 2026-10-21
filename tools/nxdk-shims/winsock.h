#pragma once
/* LookingGlass: shim nxdk -> lwIP */
#include <lwip/sockets.h>
#include <lwip/netdb.h>
#include <lwip/inet.h>
#include <errno.h>
typedef int SOCKET;
#ifndef INVALID_SOCKET
#define INVALID_SOCKET (-1)
#endif
#ifndef SOCKET_ERROR
#define SOCKET_ERROR (-1)
#endif
#define closesocket(s)        lwip_close(s)
#define ioctlsocket(s,c,a)    lwip_ioctl((s),(c),(a))
#define WSAGetLastError()     (errno)
#define WSAEWOULDBLOCK        EWOULDBLOCK
#define WSAEINPROGRESS        EINPROGRESS
#define WSAECONNREFUSED       ECONNREFUSED
#define WSAENOTCONN           ENOTCONN
typedef struct { int dummy; } WSADATA;
static inline int WSAStartup(int v, WSADATA *d) { (void)v; (void)d; return 0; }
static inline int WSACleanup(void) { return 0; }

/* LG-WIN32-TYPES */
#include <stdlib.h>
typedef unsigned long  u_long;
typedef struct hostent     HOSTENT, *PHOSTENT, *LPHOSTENT;
typedef struct in_addr     IN_ADDR, *PIN_ADDR;
typedef struct sockaddr    SOCKADDR, *PSOCKADDR;
typedef struct sockaddr_in SOCKADDR_IN, *PSOCKADDR_IN;
#ifndef MB_OK
#define MB_OK 0
#endif
static inline int MessageBoxA(void *h, const char *t, const char *c, unsigned int u)
{ (void)h; (void)t; (void)c; (void)u; return 0; }
#ifndef _atoi64
#define _atoi64(s) strtoll((s), NULL, 10)
#endif

/* LG-WIN32-STUBS */
#include <windows.h>
#include <string.h>
#ifndef CreateSemaphoreA
#define CreateSemaphoreA CreateSemaphore
#endif
static inline BOOL lg_TerminateThread(HANDLE h, DWORD c) { (void)h; (void)c; return FALSE; }
#define TerminateThread lg_TerminateThread
#ifndef MAKEWORD
#define MAKEWORD(a,b) ((unsigned short)(((unsigned char)(a)) | ((unsigned short)((unsigned char)(b))) << 8))
#endif
#define MB_ABORTRETRYIGNORE 0x00000002L
#define MB_ICONHAND         0x00000010L
#define MB_TASKMODAL        0x00002000L
#define MB_SETFOREGROUND    0x00010000L
#define IDABORT 3
#define IDRETRY 4
#define IDIGNORE 5
static inline int lg_gethostname(char *n, int l) { if (!n || l < 5) return -1; strcpy(n, "xbox"); return 0; }
#define gethostname lg_gethostname
typedef void *HKEY, **PHKEY;
#define HKEY_CURRENT_USER  ((HKEY)(uintptr_t)0x80000001)
#define HKEY_LOCAL_MACHINE ((HKEY)(uintptr_t)0x80000002)
#define REG_SZ 1
#ifndef KEY_READ
#define KEY_READ 0x20019
#endif
static inline LONG RegOpenKeyExA(HKEY k, const char *s, DWORD o, DWORD m, PHKEY r)
{ (void)k; (void)s; (void)o; (void)m; if (r) *r = NULL; return 2; }
static inline LONG RegQueryValueExA(HKEY k, const char *n, DWORD *r, DWORD *t, unsigned char *d, DWORD *c)
{ (void)k; (void)n; (void)r; (void)t; (void)d; (void)c; return 2; }
static inline LONG RegCloseKey(HKEY k) { (void)k; return 0; }
