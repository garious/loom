#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdint.h>
#define KEY_SIZE 32
enum packet_type {
    INVALID = 0,
    TX = 1,
    VERIFICATION = 2,
    __LAST = 0x7fffffff    
};

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
    uint32_t amount;
    uint32_t fee;
    uint8_t signature[KEY_SIZE];
};

struct account {
    union {
        off_t offset;
        uint8_t key[KEY_SIZE];
    } addr;
    uint64_t bal;
}

#endif // PROTOCOL_H
