#include <stdint.h>
#include <stdio.h>
#include <assert.h>
#include <sys/time.h>

#ifndef sha256 
#define sha256 sha256_avx
#endif

void sha256(void *input_data, uint32_t digest[8], uint32_t num_blks);
int main(int argc, char *argv[]) {
    static const uint32_t ostate[8] = {0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19};
    uint32_t state[8] = {0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19};
    uint8_t block[64] = "AnatolyYakovenko11/2/201712pmPSTAnatolyYakovenko11/2/201712pmPST";
    uint32_t *blkptr = (void*)block;
    uint64_t i=0;
    FILE *f = fopen(argv[1], "a+");
    struct timeval  start, now;
    if(!fseek(f, -40, SEEK_END)) {
    	assert(8 == fread(&i, 1, 8, f));
        i = i<<20;
    	assert(32 == fread(blkptr, 1, 32, f));
        blkptr[8] =  blkptr[0];
        blkptr[9] =  blkptr[1];
        blkptr[10] = blkptr[2];
        blkptr[11] = blkptr[3];
        blkptr[12] = blkptr[4];
        blkptr[13] = blkptr[5];
        blkptr[14] = blkptr[6];
        blkptr[15] = blkptr[7];
    	assert(0 == fseek(f, 0, SEEK_END));
	}
    printf("block %04x%04x%04x%04x\n", blkptr[0], blkptr[1], blkptr[2], blkptr[3]);
    printf("      %04x%04x%04x%04x\n", blkptr[4], blkptr[5], blkptr[6], blkptr[7]);
    printf("      %04x%04x%04x%04x\n", blkptr[8], blkptr[9], blkptr[10], blkptr[11]);
    printf("      %04x%04x%04x%04x\n", blkptr[12], blkptr[13], blkptr[14], blkptr[15]);
    printf("state %04x%04x%04x%04x\n", state[0], state[1], state[2], state[3]);
    printf("      %04x%04x%04x%04x\n", state[4], state[5], state[6], state[7]);
    assert(!gettimeofday(&start, 0));
    for(i; ;++i) {
        if(__builtin_expect((i & 0xfffff) == 0, 0)) {
            double total;
            uint64_t ix = i >> 20;
    		assert(!gettimeofday(&now, 0));
            total = now.tv_usec + (double)now.tv_sec * 1000000 ;
            total = total - (start.tv_usec + (double)start.tv_sec * 1000000);
			fwrite(&ix, 8, 1, f);
			fwrite(blkptr, 4, 8, f);
			fflush(f);
    		printf("block %04x%04x%04x%04x\n", blkptr[0], blkptr[1], blkptr[2], blkptr[3]);
    		printf("      %04x%04x%04x%04x\n", blkptr[4], blkptr[5], blkptr[6], blkptr[7]);
            printf("      %04x%04x%04x%04x\n", blkptr[8], blkptr[9], blkptr[10], blkptr[11]);
            printf("      %04x%04x%04x%04x\n", blkptr[12], blkptr[13], blkptr[14], blkptr[15]);
    		printf("speed %lu %G %G\n", i, total, i/total);
        }
		sha256(block, state, 1);
        blkptr[0] = state[0];
        blkptr[1] = state[1];
        blkptr[2] = state[2];
        blkptr[3] = state[3];
        blkptr[4] = state[4];
        blkptr[5] = state[5];
        blkptr[6] = state[6];
        blkptr[7] = state[7];
        blkptr[8] =  state[0];
        blkptr[9] =  state[1];
        blkptr[10] = state[2];
        blkptr[11] = state[3];
        blkptr[12] = state[4];
        blkptr[13] = state[5];
        blkptr[14] = state[6];
        blkptr[15] = state[7];

        state[0] = ostate[0];
        state[1] = ostate[1];
        state[2] = ostate[2];
        state[3] = ostate[3];
        state[4] = ostate[4];
        state[5] = ostate[5];
        state[6] = ostate[6];
        state[7] = ostate[7];

    }
}

