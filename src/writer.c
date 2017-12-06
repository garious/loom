struct state {
    sem_t sem;
    sem_t waiter;
    size_t cnt;
    struct tx_state *tx_state;
    size_t ix;
    uint64_t total_writes;
};

LOCAL int write_tx(int fd, struct account_pos *s) {
    int err = 0;
    TEST(err, (off_t)-1 != lseek(fd, s->pos, SEEK_SET));
    TEST(err, sizeof(s->acc) == write(fd, &s->acc, sizeof(s->acc)));
CATCH(err):
    return err;
}

LOCAL void *run_writer(void *ctx) {
    struct state *state = (struct state *)ctx;
    const char *table = config_get("LOOM_TABLE");
	int err = 0;
    int fd = 0;
    C_ASSERT(sizeof(offset) == 8);
	TEST(err, 0 != (fd = open(table, O_WRONLY | O_NOATIME, 0)));
    while(1) {
        struct packet *p;
        struct tx_state *s;
        uint64_t ix;
        TEST(err, !sem_wait(&state->sem));
        ix = __sync_fetch_and_add(&state->ix, 1);
        if(ix >= state->cnt) {
            sem_post(&state->waiter);
            continue;
        }
        s = &state.tx_state[ix];
        if(!s->executed) {
            continue;
        }
        TEST(err, write_tx(fd, s));
        __sync_fetch_and_add(&state->total_writes, 1);
    }
CHECK(err):
    return 0;
}

