#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <netdb.h>
#include <sys/socket.h>
#include <netinet/in.h>

#include "err.h"

PRIVATE int sock_udp_server(const char *hostname, const char *port, int *pfd) {
    int err = 0;
    struct addrinfo hints = {};
    struct addrinfo* res = 0;
    int fd = -1;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_DGRAM;
    hints.ai_protocol = 0;
    hints.ai_flags = AI_PASSIVE|AI_ADDRCONFIG;
    TEST(err, !getaddrinfo(hostname, port, &hints, &res));
    TEST(err, -1 != (fd=socket(res->ai_family,
                               res->ai_socktype,
                               res->ai_protocol)));
    TEST(err, -1 != bind(fd,res->ai_addr,res->ai_addrlen));
    *pfd = fd;
    fd = -1;
CATCH(err):
    if(fd != -1) {
        close(fd);
    }
    if(res) {
        freeaddrinfo(res);
    }
    return err;
}

PRIVATE int sock_msghdr_alloc(size_t cnt, struct msgshdr **pmsgs) {
    int err = 0;
    ssize_t i;
    struct msghdr *msgs = 0;
    struct iovec *iov;
    struct sockaddr_storage *src_addr;
    iov = (struct iovec *)&msgs[cnt];
    src_addr = (struct sockaddr_storage *)&iov[cnt];
    size_t total = (uintptr_t)&src_addr[cnt];
    TEST(err, msgs = calloc(1, total));
    iov = &msgs[cnt];
    src_addr = &iov[cnt];
    for(i = 0; i < cnt; ++i) {
        msgs[i].msg_iov=&iov[i];
        msgs[i].msg_iovlen=1;
        msgs[i].msg_name=&src_addr[i];
        msgs[i].msg_namelen=sizeof(src_addr[i]);
        msgs[i].msg_control=0;
        msgs[i].msg_controllen=0;
    }
    *pmsgs = msgs;
CATCH(err):
    return err;
}

PRIVATE int sock_msghdr_free(struct msgshdr *msgs) {
    free(msgs);
}

PRIVATE void sock_msghdr_set(struct msghdr *msgs, ssize_t cnt, char *buf[],
                             size_t size) {
    ssize_t i;
    for(i = 0; i < cnt; ++i) {
        msgs[i].msgs_iov[0].iov_base = buf[i];
        msgs[i].msgs_iov[0].iov_len = size;
    }
}

PRIVATE int sock_recvmsgs(int fd, struct msghdr *msgs, size_t cnt, 
                          uint32_t mstimeout, ssize_t *pcnt) {
    int err = 0;
    ssize_t cnt;
    struct timespec tm;
    tm.tv_sec = mstimeout / 1000;
    tm.tv_nsec = (mstimeout - (tm.tv_sec * 1000))*1000*1000;
    TEST(err, -1 != (cnt = recvmmsg(fd, msgs, cnt, 0, )));
    for(i = 0; i < cnt; ++i) {
        TEST(err, !(msgs[i].msg_flags & MSG_TRUNC));
    }
    *pcnt = cnt;
CATCH(err):
    return err;
}
