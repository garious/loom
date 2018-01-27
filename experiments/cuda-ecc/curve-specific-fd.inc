
#ifndef _UECC_CURVE_SPECIFIC_FD_H_
#define _UECC_CURVE_SPECIFIC_FD_H_

#if uECC_SUPPORTS_secp160r1

#if (uECC_OPTIMIZATION_LEVEL > 0)
static __host__ __device__ void vli_mmod_fast_secp160r1(uECC_word_t *result, uECC_word_t *product, uECC_Curve_t curve);
#endif // OPT

#endif // SUPPORTS_secp160r1


#endif
