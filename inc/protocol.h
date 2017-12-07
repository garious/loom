#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdint.h>
#include <stdbool.h>
#define KEY_SIZE 32
enum packet_type {
    INVALID = 0,
    TX = 1,
    VERIFICATION = 2,
    __LAST = 0x7fffffff    
};

//physical on the network
struct tx {
    union {
        off_t offset;
        uint8_t key[KEY_SIZE];
    } from;
    uint8_t lastvalidhash[KEY_SIZE];
    union {
        off_t offset;
        uint8_t key[KEY_SIZE];
    } to;
    //relying on these to be uint32_t's so we dont overflow
    //the uint64_t balances
    uint32_t amount;
    uint32_t fee;
    uint8_t signature[KEY_SIZE];
};

//phisical on disk
struct account {
    union {
        off_t offset;
        uint8_t key[KEY_SIZE];
    } addr;
    uint64_t bal;
};

//virtual
struct account_pos {
    struct account acc;
    int64_t change;
    off_t pos;
};

//virtual
struct tx_state {
    struct account_pos from;
    struct account_pos to;
    bool executed;
};


#endif // PROTOCOL_H
