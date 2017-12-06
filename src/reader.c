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
#include <sys/socket.h>
#include <sys/fcntl.h>


//#define SHOW_DEBUG
#include "err.h"
#include "sock.h"

#define PSIZE (512*1024)
//read packets from the network
struct packet {
    enum packet_type type;
    union {
        struct tx tx;
    } data;
};

struct packets {
    size_t cnt;
    struct packet packets[PSIZE];
};

#define QSIZE 10
struct queue {
    uint64_t read;
    uint64_t write;
    struct packets data[QSIZE];
};

static queue queue;
LOCAL int read_packets(int server) {
    int err = 0;
    struct msgshdr *pmsgs = 0;
    TEST(err, !sock_msghdr_alloc(cnt, *pmsgs));
    while(1) {
        size_t cnt = 0, i;
        uint64_t wix = __sync_fetch_and_add(&queue.write, 1);
        assert(queue.read - wix < QSIZE);
        TEST(err, !sock_msghdr_set(pmsgs, PSIZE, queue.data[wix].packets,
                                   sizeof(queue.data[wix].packets[0])));
        TEST(err, !sock_recvmsgs(server, pmsgs, PSIZE, 500,
                                 &queue.data[wix].cnt));
        assert(queue.read - wix < QSIZE);
        for(i = 0; i < cnt; ++i) {
            if(msgs[i].msg_flags & MSG_TRUNC) {
                queue.data[wix].packets[i].type = INVALID;
            }
        }
    }
CHECK(err);
    if(pmsgs) {
        sock_msghdr_free(pmsgs);
    }
    return err;
}


