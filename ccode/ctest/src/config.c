#define _GNU_SOURCE
#include <stdlib.h>
#include <string.h>
#include <wordexp.h>
#include "config.h"
#include "visibility.h"
#include "err.h"

LOCAL const struct {
    const char *var;
    const char *val;
} vars[] = {
    {"HELLO", "world"},
    {"FOO", "~/bar"},
    {0,0}
};

PRIVATE int config_expand(const char *var, char **po) {
    int err = 0;
    wordexp_t p = {};
    size_t len = 0;
    size_t i;
    char *rv;
    TEST(err, !wordexp(var, &p, 0));
    for(i = p.we_offs; i < p.we_wordc; ++i) {
        len += strlen(p.we_wordv[i]);
    }
    TEST(err, 0 != (rv = calloc(len + 1, 1)));
    *po = rv;
    for(i = p.we_offs; i < p.we_wordc; ++i) {
        len = strlen(p.we_wordv[i]);
        memmove(rv, p.we_wordv[i], len); 
        rv += len;
    }
CHECK(err):
    if(p.we_wordc) {
        wordfree(&p);
    }
    return err;
}

PRIVATE const char* config_get(const char *env) {
    int err = 0;
    int i;
    const char * val = getenv(env);
    char *exp = 0;
    if(!val) { 
        for(i = 0; vars[i].var != 0 && val == 0; ++i) {
            if(!strcmp(env, vars[i].var)) {
                val = vars[i].val;
            }
        }
        TEST(err, 0 != val);
        TEST(err, !config_expand(val, &exp));
        TEST(err, !setenv(env, exp, 1));
        val = getenv(env);
    }
CHECK(err):
    if(err) {
        val = 0;
    }
    FREEIF(exp);
    return val;
}

