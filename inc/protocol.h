#ifndef PROTOCOL_H
#define PROTOCOL_H

#include <stdint.h>

enum packet_type {
    INVALID = 0,
    TX = 1,
    VERIFICATION = 2,
    __LAST = 0x7fffffff    
};

struct tx {
    uint8_t from[32];
    uint8_t lastvalidhash[32];
    uint8_t to[32];
    uint32_t amount;
    uint32_t fee;
    uint8_t signature[32];
};

#endif // PROTOCOL_H
