#pragma once
/* LookingGlass: shim nxdk ws2tcpip.h */
#include <winsock.h>
#include <lwip/netdb.h>
#include <lwip/sockets.h>
#include <string.h>
#define ADDRESS_FAMILY sa_family_t
#ifndef NI_NUMERICHOST
#define NI_NUMERICHOST 0x02
#endif
#ifndef NI_MAXHOST
#define NI_MAXHOST 1025
#endif
#ifndef IPV6_MULTICAST_IF
#define IPV6_MULTICAST_IF 9
#endif
#ifdef __cplusplus
extern "C" {
#endif
static inline int getnameinfo(const struct sockaddr *sa, socklen_t len, char *host, socklen_t hl, char *serv, socklen_t sl, int flags) {
  (void)len; (void)flags;
  if (serv && sl) serv[0] = 0;
  if (!sa || sa->sa_family != AF_INET || !host) return EAI_FAIL;
  return inet_ntop(AF_INET, &((const struct sockaddr_in *)sa)->sin_addr, host, hl) ? 0 : EAI_FAIL;
}
static inline const char *gai_strerror(int e) { (void)e; return "getaddrinfo error (lwIP)"; }
#ifdef __cplusplus
}
#endif
