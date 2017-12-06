#include <pthread.h>
#include <semaphore.h>

#include "config.h"
#include "protocol.h"
#include "err.h"

struct state {
    sem_t sem;
    sem_t waiter;
    size_t cnt;
    struct packets *packets;
    struct tx_state *tx_state;
    size_t fix;
    uint64_t total_fetches;
};

LOCAL int find_tx(int fd, off_t offset, uint8_t key[KEY_SIZE], struct account *s) {
    int err = 0;
    static const uint8_t empty[KEY_SIZE] = {};
    off_t start = offset;
    TEST(err, (off_t)-1 != lseek(fd, start, SEEK_SET));
    while(1) {
        TEST(err, sizeof(s->from) == read(fd, &s->from, sizeof(s->from)));
        if(!memcmp(s->addr.key, key, KEY_SIZE)) {
            break;
        }
        if(!memcmp(s->addr.key, empty, KEY_SIZE)) {
            assert(s->bal == 0);
            break;
        }
        if(s->addr.offset == start) {
            s->bal = 0;
            break;
        }
        start += sizeof(s->from);
    }
CATCH(err):
    return err;
}

LOCAL void *run_fetcher(void *ctx) {
    struct state *state = (struct state *)ctx;
    const char *table = config_get("LOOM_TABLE");
	int err = 0;
    int fd = 0;
    C_ASSERT(sizeof(offset) == 8);
	TEST(err, 0 != (fd = open(table, O_RDWR | O_NOATIME, 0)));
    while(1) {
        struct packet *p;
        struct tx_state *s;
        uint64_t ix;

        TEST(err, !sem_wait(&state->sem));
        ix = __sync_fetch_and_add(&state->fix, 1);
        if(ix >= state->cnt) {
            sem_post(&state->waiter);
            continue;
        }
        p = &state.packets[ix];
        s = &state.tx_state[ix];
        TEST(err, !find_tx(fd, tx->from.offset, tx->from.key, &s->from));
        TEST(err, !find_tx(fd, tx->to.offset, tx->to.key, &s->to));
        __sync_fetch_and_add(&state->total_fetches, 2);
    }
CHECK(err):
    return 0;
}

