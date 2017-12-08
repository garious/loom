#include "hashtable.h"
#include "protocol.h"

struct state {
    sem_t sem;
    sem_t waiter;
    size_t acnt;
    //hashtable of all the loaded accounts
    struct account_pos *accounts;
    size_t pcnt;
    struct packets *packets;
    size_t ix;
    uint64_t total_executed;
};

LOCAL void *run_executor(void *ctx) {
    struct state *state = (struct state *)ctx;
	int err = 0;
    int fd = 0;
    C_ASSERT(sizeof(offset) == 8);
    while(true) {
        struct packet *p;
        struct account_pos *from, *to;
        uint64_t ix;
        uint64_t cost;

        TEST(err, !sem_wait(&state->sem));
        ix = __sync_fetch_and_add(&state->ix, 1);
        if(ix >= state->pcnt) {
            sem_post(&state->waiter);
            continue;
        }
        p = &state.packets[ix];
        if(p->type != TX) {
            continue;
        }
        cost = (uint64_t)p->fee + (uint64_t)p->amount;
        from = find(state->accounts, sizeof(state->accounts[0]), state->acnt,
                    p->from.offset);
        assert(from);
        to = find(state->accounts, sizeof(state->accounts[0]), state->acnt,
                    p->to.offset);
        assert(to);
        if(from->acc.bal + from->change < cost) {
            //skip this one if it can't afford it
            p->type = INVALID;
            continue;
        }
        from->change -= cost;
        to->change += p->amount;
        __sync_fetch_and_add(&state->total_executed, 1);
    }
CHECK(err):
    return 0;
}
