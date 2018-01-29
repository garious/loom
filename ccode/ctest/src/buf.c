#include <string.h>
#include <assert.h>
#include <stdio.h>
#include <stdarg.h>
#include <stdint.h>

#include "buf.h"
#include "util.h"

PRIVATE void buf_init(struct buf *b, void *start, size_t s) {
    char *end = (char*)start + s;
    b->mstart = start;
    b->mend = end;
    b->wstart = start;
    b->wend = start;
}

PRIVATE void buf_write(struct buf *b, const void *what, size_t n) {
    if(buf_left(b)) {
        void *where = b->wend;
        size_t max = buf_left(b);
        memmove(where, what, MIN(max, n));
    }
    b->wend += n;
}

PRIVATE void buf_alloc(struct buf *b, size_t size, size_t align, struct buf *a) {
    char *r;
    size_t s;
    buf_align(b, align);
    r = buf_get(b);
    s = buf_left(b);
    buf_advance(b, size);
    buf_init(a, r, MIN(s, size));
}

PRIVATE void buf_printf0(struct buf *b, const char *format, ...) {
    size_t s = 0;
    char *p = 0;
    va_list ap;
    int n;
    va_start(ap, format);
    if(buf_left(b)) {
        s = buf_left(b);
        p = buf_get(b);
    }
    n = vsnprintf(p, s, format, ap);
    va_end(ap);
    buf_advance(b, n + 1);
}

PRIVATE void buf_align(struct buf *b, size_t s) {
     
    b->wend = (char*)((((uintptr_t)b->wend) + (((uintptr_t)s) - 1)) & (~(((uintptr_t)s) - 1)));
}

PRIVATE void* buf_get(struct buf *b) {
    return b->wend;
}

PRIVATE void buf_advance(struct buf *b, size_t s) {
    b->wend += s;
}

PRIVATE size_t buf_required(struct buf *b) {
    return b->wend - b->wstart;
}

PRIVATE size_t buf_left(struct buf *b) {
    if(b->mend < b->wend) {
        return 0;
    }
    return b->mend - b->wend;
}
