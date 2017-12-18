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

//verify the signatures
//fetch all the data
//do all the account transfers
//write all the data
//we need to make sure we dont read and write at the same time
//i think thats where the perf hit comes from
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
