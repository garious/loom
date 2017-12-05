#define _GNU_SOURCE
#define SHOW_LG
#include "toaster.h"
#include "log.h"

LOCAL int gcnt;
LOCAL int gset;

PRIVATE int toaster_check(void) {
   if(gset && --gcnt < 0) {
       return -1;
   }
   return 0;
}

PRIVATE void toaster_set(int cnt) {
    gcnt = cnt;
    gset = 1;
}

PRIVATE int toaster_get(void) {
    if(gset) {
        return gcnt;
    }
    return -1;
}

PRIVATE void toaster_end(void) {
    gcnt = 0;
    gset = 0;
}

PRIVATE int toaster_run__(int (*test)(void)) {
    return test();
}

PRIVATE int toaster_run(int max, int (*test)(void)) {
    return toaster_run_(0, max, test);
}

PRIVATE int toaster_run_(int min, int max, int (*test)(void)) {
    int i;
    int err = -1;
    for(i = min; i <= max && err != 0; ++i) {
        LG("test count: %d", i);
        toaster_set(i);
        err = test(); 
    }
    toaster_end();
    return err;
}
