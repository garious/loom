#ifndef BUF_H
#define BUF_H

#include <sys/types.h>

#include "visibility.h"

struct buf {
    char *mstart;
    char *mend;
    char *wstart;
    char *wend;
};

PRIVATE void buf_init(struct buf *b, void *start, size_t s);
PRIVATE void buf_alloc(struct buf *b, size_t size, size_t align, struct buf *a);
PRIVATE void buf_align(struct buf *b, size_t s);
PRIVATE void buf_advance(struct buf *b, size_t s);
PRIVATE void* buf_get(struct buf *b);
PRIVATE void buf_write(struct buf *b, const void *what, size_t n);

#define buf_scalar(b, sc) do {\
    __typeof__(sc) _sc = sc; \
    buf_write(b, &(_sc), sizeof(_sc)); \
} while(0)

PRIVATE void buf_printf0(struct buf *b, const char *format, ...);
PRIVATE size_t buf_required(struct buf *b);
PRIVATE size_t buf_left(struct buf *b);

#endif // BUF_H
