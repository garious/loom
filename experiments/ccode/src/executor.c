#include "hashtable.h"
#include "protocol.h"

struct state {
    sem_t sem;
    sem_t waiter;
    size_t cnt;
    //hashtable of all the loaded accounts
    struct tx_state *states;
    struct packet *packets;
    size_t ix;
    uint64_t deposits;
    uint64_t withdrawals;
};

LOCAL void *withdrawals(void *ctx) {
    struct state *state = (struct state *)ctx;
	int err = 0;
    int fd = 0;
    C_ASSERT(sizeof(offset) == 8);
    while(true) {
        struct packet *p;
        struct tx_state *s;
        uint64_t ix;
        uint64_t cost;

        ix = __sync_fetch_and_add(&state->ix, 1);
        if(ix >= state->cnt) {
            sem_post(&state->waiter);
            TEST(err, !sem_wait(&state->sem));
            continue;
        }
        p = &state.packets[ix];
        if(p->type != TX) {
            continue;
        }
        s = &state.states[ix];
        cost = (uint64_t)p->fee + (uint64_t)p->amount;
        if(from->acc.bal + from->change < cost) {
            //skip this one if it can't afford it
            p->type = INVALID;
            continue;
        }
        from->change -= cost;
        __sync_fetch_and_add(&state->withdrawals, 1);
    }
CHECK(err):
    return 0;
}

LOCAL void *deposits(void *ctx) {
    struct state *state = (struct state *)ctx;
	int err = 0;
    int fd = 0;
    C_ASSERT(sizeof(offset) == 8);
    while(true) {
        struct packet *p;
        struct tx_state *s;
        uint64_t ix;
        uint64_t cost;

        ix = __sync_fetch_and_add(&state->ix, 1);
        if(ix >= state->pcnt) {
            sem_post(&state->waiter);
            TEST(err, !sem_wait(&state->sem));
            continue;
        }
        p = &state.packets[ix];
        if(p->type != TX) {
            continue;
        }
        s = &state.states[ix];
        to->change += p->amount;
        __sync_fetch_and_add(&state->deposits, 1);
    }
CHECK(err):
    return 0;
}
