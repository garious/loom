#ifndef TOASTER_H
#define TOASTER_H

#include "visibility.h"

PRIVATE int toaster_check(void);
PRIVATE void toaster_set(int cnt);
PRIVATE int toaster_get();
PRIVATE void toaster_end(void);
PRIVATE int toaster_run(int max, int (*test)(void));
PRIVATE int toaster_run_(int min, int max, int (*test)(void));
PRIVATE int toaster_run__(int (*test)(void));

#endif //TOASTER_H
