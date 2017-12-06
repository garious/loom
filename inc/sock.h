#ifndef SOCK_H
#define SOCK_H

PRIVATE int sock_udp_server(const char *hostname, const char *port, int *pfd);

PRIVATE int sock_msghdr_alloc(ssize_t cnt, struct msgshdr **msgs);
PRIVATE int sock_msghdr_free(struct msgshdr *msgs);
PRIVATE void sock_msghdr_set(struct msghdr *msgs, ssize_t cnt, char *buf[],
                             ssize_t size);
PRIVATE int sock_recvmsgs(int fd, struct msghdr *msgs, ssize_t cnt, 
                          uint32_t mstimeout, ssize_t *pcnt);

#endif // SOCK_H
