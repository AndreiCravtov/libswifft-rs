/*
 * Copyright (C) 2020 Yaron Gvili and Gvili Tech Ltd.
 *
 * See the accompanying LICENSE.txt file for licensing information.
 */
/*! \file src/swifft_keygen.cpp
 * \brief LibSWIFFT internal C code generation
 */
#include <iostream>
#include <fstream>
#include <iomanip>
#include "swifft.h"

#undef SWIFFT_ISET
#define SWIFFT_ISET() SWIFFT_INSTRUCTION_SET
#include "swifft_ops.inl"


//! \brief Multipliers for SWIFFT key.
static SWIFFT_ALIGN int16_t multipliers[SWIFFT_N];
//! \brief FFT table for SWIFFT key.
static SWIFFT_ALIGN int16_t fftTable[SWIFFT_V*SWIFFT_V*SWIFFT_W];

//! \brief SWIFFT key.
//! The key (A's) we use in SWIFFT shall be random elements of Z_257.
//! We generated these A's from the decimal expansion of PI as follows:  we converted each
//! triple of digits into a decimal number d. If d < (257 * 3) we used (d % 257) for the next A
//! element, otherwise move to the next triple of digits in the expansion. This guarantees that
//! the A's are random, provided that PI digits are.
static SWIFFT_ALIGN int16_t PI_key[SWIFFT_M*SWIFFT_N] = {
	141,  78, 139,  75, 238, 205, 129, 126,  22, 245, 197, 169, 142, 118, 105,  78,
	 50, 149,  29, 208, 114,  34,  85, 117,  67, 148,  86, 256,  25,  49, 133,  93,
	 95,  36,  68, 231, 211, 102, 151, 128, 224, 117, 193,  27, 102, 187,   7, 105,
	 45, 130, 108, 124, 171, 151, 189, 128, 218, 134, 233, 165,  14, 201, 145, 134,
	 52, 203,  91,  96, 197,  69, 134, 213, 136,  93,   3, 249, 141,  16, 210,  73,
	  6,  92,  58,  74, 174,   6, 254,  91, 201, 107, 110,  76, 103,  11,  73,  16,
	 34, 209,   7, 127, 146, 254,  95, 176,  57,  13, 108, 245,  77,  92, 186, 117,
	124,  97, 105, 118,  34,  74, 205, 122, 235,  53,  94, 238, 210, 227, 183,  11,
	129, 159, 105, 183, 142, 129,  86,  21, 137, 138, 224, 223, 190, 188, 179, 188,
	256,  25, 217, 176,  36, 176, 238, 127, 160, 210, 155, 148, 132,   0,  54, 127,
	145,   6,  46,  85, 243,  95, 173, 123, 178, 207, 211, 183, 224, 173, 146,  35,
	 71, 114,  50,  22, 175,   1,  28,  19, 112, 129,  21,  34, 161, 159, 115,  52,
	  4, 193, 211,  92, 115,  49,  59, 217, 218,  96,  61,  81,  24, 202, 198,  89,
	 45, 128,   8,  51, 253,  87, 171,  35,   4, 188, 171,  10,   3, 137, 238,  73,
	 19, 208, 124, 163, 103, 177, 155, 147,  46,  84, 253, 233, 171, 241, 211, 217,
	159,  48,  96,  79, 237,  18, 171, 226,  99,   1,  97, 195, 216, 163, 198,  95,
	  0, 201,  65, 228,  21, 153, 124, 230,  44,  35,  44, 108,  85, 156, 249, 207,
	 26, 222, 131,   1,  60, 242, 197, 150, 181,  19, 116, 213,  75,  98, 124, 240,
	123, 207,  62, 255,  60, 143, 187, 157, 139,   9,  12, 104,  89,  49, 193, 146,
	104, 196, 181,  82, 198, 253, 192, 191, 255, 122, 212, 104,  47,  20, 132, 208,
	 46, 170,   2,  69, 234,  36,  56, 163,  28, 152, 104, 238, 162,  56,  24,  58,
	 38, 150, 193, 254, 253, 125, 173,  35,  73, 126, 247, 239, 216,   6, 199,  15,
	 90,  12,  97, 122,   9,  84, 207, 127, 219,  72,  58,  30,  29, 182,  41, 192,
	235, 248, 237,  74,  72, 176, 210, 252,  45,  64, 165,  87, 202, 241, 236, 223,
	151, 242, 119, 239,  52, 112, 169,  28,  13,  37, 160,  60, 158,  81, 133,  60,
	 16, 145, 249, 192, 173, 217, 214,  93, 141, 184,  54,  34, 161, 104, 157,  95,
	 38, 133, 218, 227, 211, 181,   9,  66, 137, 143,  77,  33, 248, 159,   4,  55,
	228,  48,  99, 219, 222, 184,  15,  36, 254, 256, 157, 237,  87, 139, 209, 113,
	232,  85, 126, 167, 197, 100, 103, 166,  64, 225, 125, 205, 117, 135,  84, 128,
	231, 112,  90, 241,  28,  22, 210, 147, 186,  49, 230,  21, 108,  39, 194,  47,
	123, 199, 107, 114,  30, 210, 250, 143,  59, 156, 131, 133, 221,  27,  76,  99,
	208, 250,  78,  12, 211, 141,  95,  81, 195, 106,   8, 232, 150, 212, 205, 221,
	 11, 225,  87, 219, 126, 136, 137, 180, 198,  48,  68, 203, 239, 252, 194, 235,
	142, 137, 174, 172, 190, 145, 250, 221, 182, 204,   1, 195, 130, 153,  83, 241,
	161, 239, 211, 138,  11, 169, 155, 245, 174,  49,  10, 166,  16, 130, 181, 139,
	222, 222, 112,  99, 124,  94,  51, 243, 133, 194, 244, 136,  35, 248, 201, 177,
	178, 186, 129, 102,  89, 184, 180,  41, 149,  96, 165,  72, 225, 231, 134, 158,
	199,  28, 249,  16, 225, 195,  10, 210, 164, 252, 138,   8,  35, 152, 213, 199,
	 82, 116,  97, 230,  63, 199, 241,  35,  79, 120,  54, 174,  67, 112,   1,  76,
	 69, 222, 194,  96,  82,  94,  25, 228, 196, 145, 155, 136, 228, 234,  46, 101,
	246,  51, 103, 166, 246,  75,   9, 200, 161,   4, 108,  35, 129, 168, 208, 144,
	 50,  14,  13, 220,  41, 132, 122, 127, 194,   9, 232, 234, 107,  28, 187,   8,
	 51, 141,  97, 221, 225,   9, 113, 170, 166, 102, 135,  22, 231, 185, 227, 187,
	110, 145, 251, 146,  76,  22, 146, 228,   7,  53,  64,  25,  62, 198, 130, 190,
	221, 232, 169,  64, 188, 199, 237, 249, 173, 218, 196, 191,  48, 224,   5, 113,
	100, 166, 160,  21, 191, 197,  61, 162, 149, 171, 240, 183, 129, 231, 123, 204,
	192, 179, 134,  15,  47, 161, 142, 177, 239, 234, 186, 237, 231,  53, 208,  95,
	146,  36, 225, 231,  89, 142,  93, 248, 137, 124,  83,  39,  69,  77,  89, 208,
	182,  48,  85, 147, 244, 164, 246,  68,  38, 190, 220,  35, 202,  91, 157, 151,
	201, 240, 185, 218,   4, 152,   2, 132, 177,  88, 190, 196, 229,  74, 220, 135,
	137, 196,  11,  47,   5, 251, 106, 144, 163,  60, 222, 127,  52,  57, 202, 102,
	 64, 140, 110, 206,  23, 182,  39, 245,   1, 163, 157, 186, 163,  80,   7, 230,
	 44, 249, 176, 102, 164, 125, 147, 120,  18, 191, 186, 125,  64,  65, 198, 157,
	164, 213,  95,  61,  13, 181, 208,  91, 242, 197, 158,  34,  98, 169,  91,  14,
	 17,  93, 157,  17,  65,  30, 183,   6, 139,  58, 255, 108, 100, 136, 209, 144,
	164,   6, 237,  33, 210, 110,  57, 126, 197, 136, 125, 244, 165, 151, 168,   3,
	143, 251, 247, 155, 136, 130,  88,  14,  74, 121, 250, 133,  21, 226, 185, 232,
	118, 132,  89,  64, 204, 161,   2,  70, 224, 159,  35, 204, 123, 180,  13,  52,
	231,  57,  25,  78,  66,  69,  97,  42, 198,  84, 176,  59,   8, 232, 125, 134,
	193,   2, 232, 109, 216,  69,  90, 142,  32,  38, 249,  37,  75, 180, 184, 188,
	 19,  47, 120,  87, 146,  70, 232, 120, 191,  45,  33,  38,  19, 248, 110, 110,
	 44,  64,   2,  84, 244, 228, 252, 228, 170, 123,  38, 144, 213, 144, 171, 212,
	243,  87, 189,  46, 128, 110,  84,  77,  65, 183,  61, 184, 101,  44, 168,  68,
	 14, 106, 105,   8, 227, 211, 166,  39, 152,  43,  52, 254, 197,  55, 119,  89,
	168,  65,  53, 138, 177,  56, 219,   0,  58, 121, 148,  18,  44, 100, 215, 103,
	145, 229, 117, 196,  91,  89, 113, 143, 172, 239, 249, 184, 154,  39, 112,  65,
	204,  42,  84,  38, 155, 151, 151,  16, 100,  87, 174, 162, 145, 147, 149, 186,
	237, 145, 134, 144, 198, 235, 213, 163,  48, 230,  24,  47,  57,  71, 127,   0,
	150, 219,  12,  81, 197, 150, 131,  13, 169,  63, 175, 184,  48, 235,  65, 243,
	149, 200, 163, 254, 202, 114, 247,  67, 143, 250, 126, 228,  80, 130, 216, 214,
	 36,   2, 230,  33, 119, 125,   3, 142, 237, 100,   3, 152, 197, 174, 244, 129,
	232,  30, 206, 199,  39, 210, 220,  43, 237, 221, 201,  54, 179,  42,  28, 133,
	246, 203, 198, 177,   0,  28, 194,  85, 223, 109, 155, 147, 221,  60, 133, 108,
	157, 254,  26,  75, 157, 185,  49, 142,  31, 137,  71,  43,  63,  64, 237, 148,
	237, 172, 159, 160, 155, 254, 234, 224, 140, 193, 114, 140,  62, 109, 136,  39,
	255,   8, 158, 146, 128,  49, 222,  96,  57, 209, 180, 249, 202, 127, 113, 231,
	 78, 178,  46,  33, 228, 215, 104,  31, 207, 186,  82,  41,  42,  39, 103, 119,
	123, 133, 243, 254, 238, 156,  90, 186,  37, 212,  33, 107, 252,  51, 177,  36,
	237,  76, 159, 245,  93, 214,  97,  56, 190,  38, 160,  94, 105, 222, 220, 158,
	 49,  16, 191,  52, 120,  87, 179,   2,  27, 144, 223, 230, 184,   6, 129, 227,
	 69,  47, 215, 181, 162, 139,  72, 200,  45, 163, 159,  62,   2, 221, 124,  40,
	159, 242,  35, 208, 179, 166,  98,  67, 178,  68, 143, 225, 178, 146, 187, 159,
	 57,  66, 176, 192, 236, 250, 168, 224, 122,  43, 159, 120, 133, 165, 122,  64,
	 87,  74, 161, 241,   9,  87,  90,  24, 255, 113, 203, 220,  57, 139, 197, 159,
	 31, 151,  27, 140,  77, 162,   7,  27,  84, 228, 187, 220,  53, 126, 162, 242,
	 84, 181, 223, 103,  86, 177, 207,  31, 140,  18, 207, 256, 201, 166,  96,  23,
	233, 103, 197,  84, 161,  75,  59, 149, 138, 154, 119,  92,  16,  53, 116,  97,
	220, 114,  35,  45,  77, 209,  40, 196,  71,  22,  81, 178, 110,  14,   3, 180,
	110, 129, 112,  47,  18,  61, 134,  78,  73,  79, 254, 232, 125, 180, 205,  54,
	220, 119,  63,  89, 181,  52,  77, 109, 151,  77,  80, 207, 144,  25,  20,   6,
	208,  47, 201, 206, 192,  14,  73, 176, 256, 201, 207,  87, 216,  60,  56,  73,
	 92, 243, 179, 113,  49,  59,  55, 168, 121, 137,  69, 154,  95,  57, 187,  47,
	129,   4,  15,  92,   6, 116,  69, 196,  48, 134,  84,  81, 111,  56,  38, 176,
	239,   6, 128,  72, 242, 134,  36, 221,  59,  48, 242,  68, 130, 110, 171,  89,
	 13, 220,  48,  29,   5,  75, 104, 233,  91, 129, 105, 162,  44, 113, 163, 163,
	 85, 147, 190, 111, 197,  80, 213, 153,  81,  68, 203,  33, 161, 165,  10,  61,
	120, 252,   0, 205,  28,  42, 193,  64,  39,  37,  83, 175,   5, 218, 215, 174,
	128, 121, 231,  11, 150, 145, 135, 197, 136,  91, 193,   5, 107,  88,  82,   6,
	  4, 188, 256,  70,  40,   2, 167,  57, 169, 203, 115, 254, 215, 172,  84,  80,
	188, 167,  34, 137,  43, 243,   2,  79, 178,  38, 188, 135, 233, 194, 208,  13,
	 11, 151, 231, 196,  12, 122, 162,  56,  17, 114, 191, 207,  90, 132,  64, 238,
	187,   6, 198, 176, 240,  88, 118, 236,  15, 226, 166,  22, 193, 229,  82, 246,
	213,  64,  37,  63,  31, 243, 252,  37, 156,  38, 175, 204, 138, 141, 211,  82,
	106, 217,  97, 139, 153,  56, 129, 218, 158,   9,  83,  26,  87, 112,  71,  21,
	250,   5,  65, 141,  68, 116, 231, 113,  10, 218,  99, 205, 201,  92, 157,   4,
	 97,  46,  49, 220,  72, 139, 103, 171, 149, 129, 193,  19,  69, 245,  43,  31,
	 58,  68,  36, 195, 159,  22,  54,  34, 233, 141, 205, 100, 226,  96,  22, 192,
	 41, 231,  24,  79, 234, 138,  30, 120, 117, 216, 172, 197, 172, 107,  86,  29,
	181, 151,   0,   6, 146, 186,  68,  55,  54,  58, 213, 182,  60, 231,  33, 232,
	 77, 210, 216, 154,  80,  51, 141, 122,  68, 148, 219, 122, 254,  48,  64, 175,
	 41, 115,  62, 243, 141,  81, 119, 121,   5,  68, 121,  88, 239,  29, 230,  90,
	135, 159,  35, 223, 168, 112,  49,  37, 146,  60, 126, 134,  42, 145, 115,  90,
	 73, 133, 211,  86, 120, 141, 122, 241, 127,  56, 130,  36, 174,  75,  83, 246,
	112,  45, 136, 194, 201, 115,   1, 156, 114, 167, 208,  12, 176, 147,  32, 170,
	251, 100, 102, 220, 122, 210,   6,  49,  75, 201,  38, 105, 132, 135, 126, 102,
	 13, 121,  76, 228, 202,  20,  61, 213, 246,  13, 207,  42, 148, 168,  37, 253,
	 34,  94, 141, 185,  18, 234, 157, 109, 104,  64, 250, 125,  49, 236,  86,  48,
	196,  77,  75, 237, 156, 103, 225,  19, 110, 229,  22,  68, 177,  93, 221, 181,
	152, 153,  61, 108, 101,  74, 247, 195, 127, 216,  30, 166, 168,  61,  83, 229,
	120, 156,  96, 120, 201, 124,  43,  27, 253, 250, 120, 143,  89, 235, 189, 243,
	150,   7, 127, 119, 149, 244,  84, 185, 134,  34, 128, 193, 236, 234, 132, 117,
	137,  32, 145, 184,  44, 121,  51,  76,  11, 228, 142, 251,  39,  77, 228, 251,
	 41,  58, 246, 107, 125, 187,   9, 240,  35,   8,  11, 162, 242, 220, 158, 163,
	  2, 184, 163, 227, 242,   2, 100, 101,   2,  78, 129,  34,  89,  28,  26, 157,
	 79,  31, 107, 250, 194, 156, 186,  69, 212,  66,  41, 180, 139,  42, 211, 253,
	256, 239,  29, 129, 104, 248, 182,  68,   1, 189,  48, 226,  36, 229,   3, 158,
	 41,  53, 241,  22, 115, 174,  16, 163, 224,  19, 112, 219, 177, 233,  42,  27,
	250, 134,  18,  28, 145, 122,  68,  34, 134,  31, 147,  17,  39, 188, 150,  76,
};


//! \brief Centers a mod-257 number around 0.
//! \param[in] x the mod-257 number.
//! \returns x - 257 if x > 257/2, x + 257 if x < -257/2, otherwise x.
static int Center(int x)
{
	int result = x % SWIFFT_P;

	if (result > (SWIFFT_P / 2))
		result -= SWIFFT_P;

	if (result < (SWIFFT_P / -2))
		result += SWIFFT_P;

	return result;
}

//! \brief Reverses bits.
//! \param[in] input the bits to reverse.
//! \param[in] numOfBits 1 shifted to the number of bits to reverse
//! \returns the reversed bits.
static int ReverseBits(int input, int numOfBits)
{
	int reversed = 0;

	for (input |= numOfBits; input > 1; input >>= 1)
		reversed = (reversed << 1) | (input & 1);

	return reversed;
}

//! \brief Initializes the key along with the related multipliers and FFT table.
static void SWIFFT_Initialize()
{
	int i, j, k, w, x;
	// The powers of OMEGA
	int omegaPowers[2 * SWIFFT_N + 1];
	int8_t reverseBits[SWIFFT_N];
	omegaPowers[0] = 1;

	for (i = 1; i <= (2 * SWIFFT_N); ++i)
	{
		omegaPowers[i] = Center(omegaPowers[i - 1] * OMEGA);
	}

	for (i = 0; i < SWIFFT_N/SWIFFT_W; ++i) {
		reverseBits[i] = ReverseBits(i,SWIFFT_N/SWIFFT_W);
	}

	for (i = 0; i < (SWIFFT_N / SWIFFT_W); ++i)
	{
		for (j = 0; j < SWIFFT_W; ++j)
		{
			multipliers[(i << SWIFFT_LOG2_W) + j] = omegaPowers[reverseBits[i] * (2 * j + 1)];
		}
	}

	for (i = 0; i < SWIFFT_W; ++i) {
		reverseBits[i] = ReverseBits(i,SWIFFT_W);
	}

	for (w = 0; w < SWIFFT_V; w++)
	{
		for (x = 0; x < SWIFFT_V; ++x)
		{
			for (j = 0; j < SWIFFT_N/8; ++j)
			{
				int temp = 0;
				for (k = 0; k < SWIFFT_LOG2_V; ++k)
				{
					int value = omegaPowers[((SWIFFT_N/8) * (2 * j + 1) * reverseBits[k]) % (2 * SWIFFT_N)] * ((x >> k) & 1);
					temp += ((w >> k) & 1) == 0 ? value : - value;
				}

				fftTable[(SWIFFT_INT16(w,x) << SWIFFT_LOG2_W) + j] = Center(temp);
			}
		}
	}

	for (j=0; j<SWIFFT_N*SWIFFT_M; j++) {
		PI_key[j] = Center(PI_key[j]);
	}
}


//! \brief Writes an array of 16-bit elements in C source-code format.
//! \param[in,out] out the output stream to write to.
//! \param[in] arr the array to write.
//! \param[in] arrlen the length of the array.
//! \param[in] arrsig a signature of the array: a suffix-name followed by dimensions specification.
void writeArray(std::ofstream & out, const int16_t * arr, size_t arrlen, const char * arrsig)
{
	out << "const SWIFFT_ALIGN int16_t SWIFFT_" << arrsig << " = {" << std::endl;
	for (size_t i=0; i<arrlen; i++) {
		if ((i & 0x7) == 0) {
			out << '\t';
		}
		out << std::right << std::setw(4) << arr[i];
		if (i < arrlen - 1) {
			out << ",";
		}
		if ((i & 0x7) == 0x7) {
			out << std::endl;
		}
	}
	out << "};" << std::endl;
}


//! \brief Writes C source code for the generated SWIFFT key into a file given as the first argument of the program.
int main(int argc, char **argv)
{
	if (argc < 2) {
		std::cerr << "Usage: " << argv[0] << " <outpath>" << std::endl;
		return 1;
	}
	SWIFFT_Initialize();
	std::ofstream out(argv[1]);
	out << std::endl;
	out << "#include \"swifft_impl.inl\"" << std::endl;
	out << std::endl;
	writeArray(out, multipliers, SWIFFT_N, "multipliers[SWIFFT_N]");
	out << std::endl;
	writeArray(out, fftTable, SWIFFT_V*SWIFFT_V*SWIFFT_W, "fftTable[SWIFFT_V*SWIFFT_V*SWIFFT_W]");
	out << std::endl;
	writeArray(out, PI_key, SWIFFT_M*SWIFFT_N, "PI_key[SWIFFT_M*SWIFFT_N]");
	return 0;
}
