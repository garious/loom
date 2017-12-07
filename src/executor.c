struct state {
    sem_t sem;
    sem_t waiter;
    size_t cnt;
    struct packets *packets;
    struct tx_state *tx_state;
    struct uint32_t *ops;
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
        struct tx_state *s;
        uint64_t ix;
        uint64_t cost;

        TEST(err, !sem_wait(&state->sem));
        ix = __sync_fetch_and_add(&state->ix, 1);
        if(ix >= state->cnt) {
            sem_post(&state->waiter);
            continue;
        }
        p = &state.packets[ix];
        s = &state.tx_state[ix];
        cost = (uint64_t)p->fee + (uint64_t)p->amount;
        s->from.change = -cost;
        s->to.change = p->amount;
        if (s->from.bal < cost) {
            //not enough cash
            continue;
        }
        if (s->to.bal < (s->to.bal + p->amount)) {
            //overflow
            continue;
        }
        s->to.bal = s->to.bal + p->amount;
        s->from.bal = s->from.bal - cost;
        s->executed = true;
        TEST(err, !find_tx(fd, tx->to.offset, tx->to.key, &s->to));
        __sync_fetch_and_add(&state->total_executed, 1);
    }
CHECK(err):
    return 0;
}
