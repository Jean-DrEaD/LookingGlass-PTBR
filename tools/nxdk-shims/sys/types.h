#pragma once
/* LookingGlass: shim nxdk sys/types.h */
#include <stddef.h>
#include <stdint.h>
#ifndef LG_SYS_TYPES
#define LG_SYS_TYPES
#ifndef _OFF_T_DEFINED
#define _OFF_T_DEFINED
typedef long off_t;
#endif
#ifndef _SSIZE_T_DEFINED
#define _SSIZE_T_DEFINED
typedef int ssize_t;
#endif
typedef unsigned int mode_t;
typedef int pid_t;
#endif

