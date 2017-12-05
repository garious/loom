#ifndef CONFIG_H
#define CONFIG_H


#include "visibility.h"

PRIVATE const char* config_get(const char *env);
PRIVATE int config_expand(const char *var, char **po);

#endif
