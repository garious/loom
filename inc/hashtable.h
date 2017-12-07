#ifndef HASHTABLE_H
#define HASHTABLE_H

#include <stdint.h>

//very dumb hashtable
//elem_size must be greater than sizeof(uint64_t)
//elem_size must 8 byte aligned
//first 8 bytes must be the key
void* find(void* array, size_t elem_size, size_t num_elems,
           uint64_t key) {
   uint64_t offset = key % num_elems;
   uint64_t i;
   for(i = offset; i < offset + num_elems; ++i) {
       uint64_t pos = (i % num_elems) * elem_size;
       uint64_t *kptr = (uint64_t*)(((uintptr_t)array) + pos);
       if(*kptr == 0) {
           return 0;
       }
       if(*kptr == key) {
           return kptr;
       }
   }
}


#endif //HASHTABLE_H
