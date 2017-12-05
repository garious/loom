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
#define QSIZE (512*1024)
//read packets from the network
//verify the signatures
//fetch all the data
//do all the account transfers
//write all the data
//compute the merkle
//sequence all the transactions 
int main(int _argc, char * const _argv[]) {
}
