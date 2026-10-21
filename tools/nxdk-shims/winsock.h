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
