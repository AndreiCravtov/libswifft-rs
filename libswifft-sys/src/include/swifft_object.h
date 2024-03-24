/*
 * Copyright (C) 2021 Yaron Gvili and Gvili Tech Ltd.
 *
 * See the accompanying LICENSE.txt file for licensing information.
 */
/*! \file include/swifft_object.h
 * \brief LibSWIFFT object public C API
 *
 * Same as swifft.h API but as function pointers. Refer to the included "*.inl"
 * files for details.
 */

#ifndef __LIBSWIFFT_SWIFFT_OBJECT_H__
#define __LIBSWIFFT_SWIFFT_OBJECT_H__

#include "swifft_common.h"

LIBSWIFFT_BEGIN_EXTERN_C

#undef LIBSWIFFT_API
#define LIBSWIFFT_API(name) (*name)

typedef struct {
#include "swifft_fft_api.inl"
} swifft_fft_object_t;

typedef struct {
#include "swifft_arith_api.inl"
} swifft_arith_object_t;

typedef struct {
#include "swifft_hash_api.inl"
} swifft_hash_object_t;

typedef struct {
	swifft_fft_object_t fft;
	swifft_arith_object_t arith;
	swifft_hash_object_t hash;
} swifft_object_t;

#undef LIBSWIFFT_API

#undef SWIFFT_ISET
#include "swifft_object_iset.inl"

#if defined(__AVX__)
        #include "swifft_avx.h"
        #undef SWIFFT_ISET
        #define SWIFFT_ISET() AVX
        #include "swifft_object_iset.inl"
#else
        #pragma message "LibSWIFFT API for AVX is disabled"
#endif

#if defined(__AVX2__)
        #include "swifft_avx2.h"
        #undef SWIFFT_ISET
        #define SWIFFT_ISET() AVX2
        #include "swifft_object_iset.inl"
#else
        #pragma message "LibSWIFFT API for AVX2 is disabled"
#endif

#if defined(__AVX512F__)
        #include "swifft_avx512.h"
        #undef SWIFFT_ISET
        #define SWIFFT_ISET() AVX512
        #include "swifft_object_iset.inl"
#else
        #pragma message "LibSWIFFT API for AVX512 is disabled"
#endif

LIBSWIFFT_END_EXTERN_C

#endif /* __LIBSWIFFT_SWIFFT_OBJECT_H__ */
