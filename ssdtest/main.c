#define _FILE_OFFSET_BITS 64
#define _GNU_SOURCE
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <limits.h>
#include <fcntl.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/fcntl.h>

//#define SHOW_DEBUG
#include "err.h"


int main(int argc, const char *argv[]) {
	int err = 0;
    int64_t i;
	int64_t size = 100*1024*1024*1024llu;
    int fd = 0;
    uint8_t buf[64];
    struct timeval  start, now;
    double total;
    int over = 0;
    if(argc > 1){
        printf("random mode\n");
    }
    TEST(err, !gettimeofday(&start, 0));
	TEST(err, 0 != (fd = open("table", O_RDWR | O_NOATIME, 0)));
    for(i = 0; i < size; i = i + 64) {
        off_t target = i;
        if(argc > 1){
            target = MAX(0, (((double)rand())/(double)RAND_MAX * (double)size) - 64);
            assert(target < (size - 64));
        }
        if(target > UINT32_MAX) {
            over++;
        }
        TEST(err, (off_t)-1 != lseek(fd, target, SEEK_SET));
        TEST(err, 64 == read(fd, buf, 64));
        memset(buf, (char)i, 64);
        TEST(err, (off_t)-1 != lseek(fd, target, SEEK_SET));
        TEST(err, 64 == write(fd, buf, 64));
    	TEST(err, !gettimeofday(&now, 0));
        total = now.tv_usec + (double)now.tv_sec * 1000000 ;
        total = total - (start.tv_usec + (double)start.tv_sec * 1000000);
        if(i % (64*1024*1024) == 0) {
    	    printf("speed %d %lu %G %G tps\n", over, i/64, total, (i/64)/total * 1000000);
        }
    }
CHECK(err):
    return err;
}
