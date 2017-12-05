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

//#define SHOW_DEBUG
#include "err.h"

#define TSIZE (512*1024*1024*1024llu)
#define QSIZE (256*1024)
struct tx {
    off_t from;
    off_t to;
    uint32_t amount;
};
static struct tx queue[QSIZE];
uint32_t rix;

#define OPSIZE (2*QSIZE)
static uint32_t ops[OPSIZE];

void *run_thread(void *ctx) {
    int *count = (int*)ctx;
	int err = 0;
    int fd = 0;
    uint8_t buf[64];
	TEST(err, 0 != (fd = open("table", O_RDWR | O_NOATIME, 0)));
    while(1) {
        uint32_t ix = __sync_fetch_and_add(&rix, 1);
        uint32_t op = ops[ix % OPSIZE];
        uint32_t tix = op;
        uint32_t flag = 0;
        if(op >= QSIZE) {
            flag = 1;
            tix = op - QSIZE;
        } 
        struct tx *tx = &queue[tix];
        if(!flag) {
            TEST(err, (off_t)-1 != lseek(fd, tx->from, SEEK_SET));
        } else {
            TEST(err, (off_t)-1 != lseek(fd, tx->to, SEEK_SET));
        }
        TEST(err, 64 == read(fd, buf, 64));
        *count = *count + 1;
    }
CHECK(err):
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
    int prev = 0;
    pthread_t thread[64] = {};
    struct timeval  start, now;
	int64_t size = TSIZE;
    double total;
    int threads = atoi(argv[1]);
    for(i = 0; i < OPSIZE; i++) {
        ops[i] = i;
    }
    for(i = 0; i < QSIZE; i++) {
        queue[i].from = MAX(0, (((double)rand())/(double)RAND_MAX * (double)size) - 64);
        queue[i].to = MAX(0, (((double)rand())/(double)RAND_MAX * (double)size) - 64);
    }
    qsort(&ops, OPSIZE, sizeof(ops[0]), compare);


    TEST(err, !gettimeofday(&start, 0));
    for(i = 0; i < threads; i++) {
        TEST(err, !pthread_create(&thread[i], 0, run_thread, &counts[i]));
    }
    while(1) {
        int sum = 0;
        int diff;
        sleep(1);
        for(i = 0; i < threads; i++) {
           sum += counts[i]; 
        }
    	TEST(err, !gettimeofday(&now, 0));
        total = now.tv_usec + (double)now.tv_sec * 1000000 ;
        total = total - (start.tv_usec + (double)start.tv_sec * 1000000);
        start = now;
        diff = sum - prev;
        prev = sum;
    	printf("speed %d %d %G %G tps\n", sum, diff, total, ((double)diff)/total * 1000000);
        //for(i = 0; i < QSIZE; i++) {
        //    queue[i].from = MAX(0, (((double)rand())/(double)RAND_MAX * (double)size) - 64);
        //    queue[i].to = MAX(0, (((double)rand())/(double)RAND_MAX * (double)size) - 64);
        //}
        //qsort(&ops, OPSIZE, sizeof(ops[0]), compare);
    }
CHECK(err):
    return err;
}
