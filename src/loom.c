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
#include "sock.h"

#define TSIZE (512*1024*1024*1024llu)
#define QSIZE (512*1024)
//read packets from the network
struct packet {
    uint8_t from[32];
    uint8_t lastvalidhash[32];
    uint8_t to[32];
    uint32_t amount;
    uint8_t signature[32];
};
LOCAL int read_packets(int server) {
    sock_recvfrom(server, &packet
}
//verify the signatures
//fetch all the data
//do all the account transfers
//write all the data
//compute the merkle
//sequence all the transactions 
PUBLIC int main(int _argc, char * const _argv[]) {
    int err = 0;
    int server = -1;
    const char *port;
    TEST(err, !(port = config_get(LOOM_PORT)));
    TEST(err, !sock_udp_server(0, port, &server));
CATCH(err):
    if(server != -1) {
        close(server);
    }
    return err;
}
