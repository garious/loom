#ifndef LOG_H
#define LOG_H

#include <stdio.h>
#include <sys/types.h>
#include <sys/syscall.h>   /* For SYS_xxx definitions */
#include <unistd.h>
#include "util.h"

#define GETTID() (long int)syscall(SYS_gettid)
#define PRINTLN(format, ...) \
do { \
    fprintf(stderr, __FILE_LINE__ ":" format "\n", ##__VA_ARGS__); \
} while(0)

#define NOOP (void)0

#ifdef SHOW_LG
#define LG(format, ...) PRINTLN("%ld:" format, GETTID(), ##__VA_ARGS__)
#else
#define LG(format, ...) NOOP
#endif

#ifdef SHOW_TRACE
#define LOG_TRACE(format, ...)  PRINTLN("trace:%ld:" format, GETTID(), ##__VA_ARGS__)
#else
#define LOG_TRACE(format, ...)  NOOP
#endif

#ifdef SHOW_DEBUG
#define LOG_DEBUG(format, ...)  PRINTLN("debug:%ld:" format, GETTID(), ##__VA_ARGS__)
#else
#define LOG_DEBUG(format, ...)  NOOP
#endif

#define LOG(lvl, format, ...) LOG_ ##lvl (format, ##__VA_ARGS__)

#endif // LOG_H
