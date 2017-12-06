#ifndef CONFIG_H
#define CONFIG_H


#include "visibility.h"

#define LOOM_PORT "10101"
#define TSIZE (512*1024*1024*1024llu)

PRIVATE const char* config_get(const char *env);
PRIVATE int config_expand(const char *var, char **po);

#endif
