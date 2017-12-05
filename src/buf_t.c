#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "buf.h"

void test_buf1(void) {
    struct buf b;
    buf_init(&b, 0, 0);
    buf_write(&b,  "hellow", 7);
    assert(buf_required(&b) == 7);
    assert(buf_left(&b) == 0);
    return;
}

void test_buf2(void) {
    char mem[11];
    struct buf b;
    buf_init(&b, mem, sizeof(mem));
    assert(buf_left(&b) == 11);
    assert(buf_required(&b) == 0);

    buf_write(&b, "hellowo", 7);

    assert(buf_required(&b) == 7);
    assert(buf_left(&b) == 4);

    buf_write(&b, "rld", 4);

    assert(buf_left(&b) == 0);
    assert(buf_required(&b) == 11);

    assert(0 == memcmp(mem, "helloworld", 11));
    buf_write(&b, "foo", 4);
    assert(buf_left(&b) == 0);
    assert(buf_required(&b) == 15);
    return;
}

void test_buf3(void) {
    struct buf b;
    buf_init(&b, 0, 0);
    buf_printf0(&b,  "%s%s", "hello","world");
    assert(buf_left(&b) == 0);
    assert(buf_required(&b) == 11);
}

void test_buf4(void) {
    struct buf b;
    char mem[10] = {};
    buf_init(&b, mem, sizeof(mem));
    buf_printf0(&b,  "%s%s", "hello","world");
    assert(buf_required(&b) == 11);
    assert(buf_left(&b) == 0);
    assert(0 == memcmp(mem, "helloworl", 10));
}

void test_buf5(void) {
    struct buf b;
    buf_init(&b, 0, 0);
    buf_write(&b, 0, 1);
    buf_align(&b, 8);
    assert(8 == (uintptr_t)buf_get(&b));
    buf_align(&b, 8);
    assert(8 == (uintptr_t)buf_get(&b));
    buf_advance(&b, 1);
    assert(9 == (uintptr_t)buf_get(&b));
    buf_align(&b, 8);
    assert(16 == (uintptr_t)buf_get(&b));
}

void test_buf6(void) {
    struct buf b, c;
    buf_init(&b, 0, 0);
    buf_advance(&b, 1);
    buf_alloc(&b, 10, 8, &c);
    assert(buf_required(&b) == 18);
    assert(8 == (uintptr_t)buf_get(&c));
}

int main(int _argc, char * const _argv[]) {
    test_buf1();
    test_buf2();
    test_buf3();
    test_buf4();
    test_buf5();
    test_buf6();
    return 0;
}
