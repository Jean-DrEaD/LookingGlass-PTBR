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
