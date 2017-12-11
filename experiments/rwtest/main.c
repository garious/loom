#define _FILE_OFFSET_BITS 64
#define _GNU_SOURCE
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <limits.h>
#include <fcntl.h>
#include <unistd.h>
#include <pthread.h>

#include <sys/time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/fcntl.h>

#define SHOW_DEBUG
#include "err.h"
#define X 16
#define TSIZE (X*40*1024*1024*1024llu)
#define QSIZE (X*128*1024llu)
#define BSIZE (X*16*1024llu)
#define BUFSIZE 32
#define OPSIZE (2llu*QSIZE)
static uint32_t *ops;

struct tx {
    off_t from;
    off_t to;
    uint32_t amount;
};
static struct tx *queue;
uint64_t rix;
static char *table;

void *run_thread(void *ctx) {
    int *count = (int*)ctx;
    while(1) {
        uint64_t ix = __sync_fetch_and_add(&rix, 1);
        uint32_t op = ops[ix % OPSIZE];
        uint32_t tix = op;
        off_t pos = 0;
        char buf[BUFSIZE] = {};
        struct tx *tx;
        int flag = 0;
        if(op >= QSIZE) {
            flag = 1;
            tix = op - QSIZE;
        } 
        memmove(buf, &table[pos], BUFSIZE);
        tx = &queue[tix];
        pos = !flag ? tx->from : tx->to;
        memset(buf, (char)ix, BUFSIZE);
        memmove(&table[pos], buf, BUFSIZE);
        *count = *count + 1;
    }
    assert(0);
    return 0;
}

int compare(const void *pa, const void *pb) {
    uint32_t *oa = (uint32_t *)pa;
    uint32_t *ob = (uint32_t *)pb;
    uint32_t aitx = *oa >= QSIZE ? *oa - QSIZE : *oa;
    uint32_t bitx = *ob >= QSIZE ? *ob - QSIZE : *ob;
    uint32_t aop = *oa >= QSIZE;
    uint32_t bop = *ob >= QSIZE;
    struct tx* atx = &queue[aitx];
    struct tx* btx = &queue[bitx];
    off_t a = aop == 0 ? atx->from : atx->to;
    off_t b = bop == 0 ? btx->from : btx->to;
    return (a > b) - (a < b);
}

int main(int argc, const char *argv[]) {
    int err = 0;
    int64_t i;
    int counts[64] = {};
    uint64_t prev = 0;
    pthread_t thread[64] = {};
    struct timeval  start, now;
    int64_t size = TSIZE;
    double total;
    int threads = atoi(argv[1]);
    table = malloc(TSIZE);
    assert(table);
    ops = calloc(1, OPSIZE *sizeof(ops[0]));
    assert(ops);
    queue = calloc(1, QSIZE *sizeof(queue[0]));
    assert(queue);
    for(i = 0; i < QSIZE; i++) {
        queue[i].from = ((off_t)(MAX(0, (((double)rand())/(double)RAND_MAX * (double)size)))) & (~(32 - 1));
        queue[i].to = ((off_t)(MAX(0, (((double)rand())/(double)RAND_MAX * (double)size)))) & (~(32 - 1));
        //assert(queue[i].from != queue[i].to);
        //assert(queue[i].from != 0);
        //assert(0 != queue[i].to);
    }
    for(i = 0; i < OPSIZE; i++) {
        ops[i] = i;
    }
    //for(i = 0; i < OPSIZE; i += BSIZE) {
    //    qsort(&ops[i], BSIZE, sizeof(ops[0]), compare);
    //}

    TEST(err, !gettimeofday(&start, 0));
    for(i = 0; i < threads; i++) {
        TEST(err, !pthread_create(&thread[i], 0, run_thread, &counts[i]));
    }
    while(1) {
        int64_t diff;
        //sleep(1);
        TEST(err, !gettimeofday(&now, 0));
        total = now.tv_usec + (double)now.tv_sec * 1000000 ;
        total = total - (start.tv_usec + (double)start.tv_sec * 1000000);
        start = now;
        diff = rix - prev;
        prev = rix;
        printf("speed %lu %ld %G %G tps\n", rix, diff, total, ((double)diff)/total * 1000000);
        for(i = 0; i < QSIZE; i++) {
            queue[i].from = ((off_t)(MAX(0, (((double)rand())/(double)RAND_MAX * (double)size)))) & (~(32 - 1));
            queue[i].to = ((off_t)(MAX(0, (((double)rand())/(double)RAND_MAX * (double)size)))) & (~(32 - 1));
            //assert(queue[i].from != queue[i].to);
            //assert(queue[i].from != 0);
            //assert(0 != queue[i].to);
        }
        //for(i = 0; i < OPSIZE; i += BSIZE) {
        //    qsort(&ops[i], BSIZE, sizeof(ops[0]), compare);
        //}
    }
CHECK(err):
    return err;
}
