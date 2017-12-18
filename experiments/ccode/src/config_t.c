#define _GNU_SOURCE
#include <assert.h>
#include <string.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include <sys/syscall.h>   /* For SYS_xxx definitions */
#include <dlfcn.h>
#include <wordexp.h>

#include "config.h"
#include "toaster.h"
#include "err.h"

int setenv(const char *name, const char *value, int overwrite) {
    int (*real)(const char *, const char *, int) = dlsym(RTLD_NEXT, "setenv");
    if(!toaster_check()) {
        return real(name, value, overwrite);
    }
    return -1;
}
char *getenv(const char *name) {
    char *(*real)(const char *) = dlsym(RTLD_NEXT, "getenv");
    if(!toaster_check()) {
        return real(name);
    }
    return 0;
}
int wordexp(const char *s, wordexp_t *p, int flags) {
    int (*real)(const char *, wordexp_t *, int) = dlsym(RTLD_NEXT, "wordexp");
    if(!toaster_check()) {
        return real(s, p, flags);
    }
    return -1;
}
void *calloc(size_t nmemb, size_t size) {
    void *(*real)(size_t, size_t) = dlsym(RTLD_NEXT, "calloc");
    if(!toaster_check()) {
        return real(nmemb, size);
    }
    return 0;
}
int strcmp3(const char *str, const char *one, const char *two) {
    int err = 0;
    size_t len = strlen(one);
    TEST(err, 0 != str);
    TEST(err, !strncmp(str, one, len));
    TEST(err, !strcmp(str + len, two));
CHECK(err):
    return err;
}

int strcmp2(const char *str, const char *one) {
    int err = 0;
    TEST(err, 0 != str);
    TEST(err, !strcmp(str, one));
CHECK(err):
    return err;
}

int test_expand() {
    int err = 0;
    char *home = 0;
    TEST(err, !config_expand("~", &home));
    assert(home != 0);
    TEST(err, !strncmp(home, "/home", strlen("/home")));

    TEST(err, !strcmp3(config_get("FOO"),  home, "/bar"));

CHECK(err):
    FREEIF(home);
    return err;
}

int test_get() {
    int err = 0;
    TEST(err, !strcmp2(config_get("HELLO"),             "world"));

    TEST(err, 0 == config_get("UNKNOWNVARfdsasfdsafdsafsdafasfds"));
CHECK(err):
    return err;
}

int main() {
    int err = 0;
    TEST(err, !toaster_run(1000, test_get));
    TEST(err, !toaster_run(1000, test_expand));
CHECK(err):
    assert(err == 0);
    return err;
}
