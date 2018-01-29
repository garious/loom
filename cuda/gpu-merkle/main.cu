#include "sha256.h"
#include <map>
#include <cuda.h>
#include <stdio.h>

std::map<int, uint32_t *> context_idata[2];
std::map<int, uint32_t *> context_odata[2];
std::map<int, cudaStream_t> context_streams[2];
std::map<int, uint32_t *> context_tstate[2];
std::map<int, uint32_t *> context_ostate[2];
std::map<int, uint32_t *> context_hash[2];

#define HASH_SIZE (8 * sizeof(uint32_t))

int main(int argc, const char* argv[]) {
    int thrd_id = 0;
    int throughput = 128;
    int stream = 0;
    uint32_t cpu_pdata[20] = {0};
    uint32_t cpu_midstate[8] = {0};

    cpu_pdata[0] = 2;

    context_idata[stream][0] = NULL;
    cudaMalloc(&context_idata[0][0], 32 * sizeof(uint32_t));
    cudaMemset(&context_idata[0][0], strtol(argv[1], nullptr, 10), 32 * sizeof(uint32_t));

    context_odata[stream][0] = NULL;
    cudaMalloc(&context_odata[0][0], 32 * sizeof(uint32_t));

    context_ostate[stream][0] = NULL;
    cudaMalloc(&context_ostate[stream][0], 32 * sizeof(uint32_t));

    context_tstate[stream][0] = NULL;
    cudaMalloc(&context_tstate[0][0], 32 * sizeof(uint32_t));
 
    context_hash[stream][0] = NULL;
    cudaMalloc(&context_hash[stream][0], 8 * sizeof(uint32_t));

    cudaStream_t cudaStream;
    cudaStreamCreate(&cudaStream);
    context_streams[stream][0] = cudaStream;

    printf("prepare_sha256:\n");
    prepare_sha256(thrd_id, cpu_pdata, cpu_midstate);
    printf("pre_sha256\n");
    pre_sha256(thrd_id, stream, 0, throughput);
    printf("post_sha256\n");
    post_sha256(thrd_id, stream, throughput);

    uint32_t* h_hash = (uint32_t*)malloc(HASH_SIZE);

    cudaMemcpy(h_hash, context_hash[stream][thrd_id], HASH_SIZE, cudaMemcpyDeviceToHost);

    cudaDeviceSynchronize();

    for (int i = 0; i < HASH_SIZE / sizeof(uint32_t); i++) {
        printf("%08x ", h_hash[i]);
    }
    printf("\n");
 
    return 0;
}
