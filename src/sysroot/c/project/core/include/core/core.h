#ifndef CORE_COMMON_H
#define CORE_COMMON_H

#include "stdint.h"
#include "stddef.h"
#include "stdbool.h"

#include "core/macro.h"

struct mem;
struct val_str;

#define U1_MAX         255
#define U2_MAX        65535
#define U4_MAX        4294967295U
#define U8_MAX        18446744073709551615ULL

typedef uint8_t u1;
typedef uint16_t u2;
typedef uint32_t u4;
typedef uint64_t u8;

typedef int8_t i1;
typedef int16_t i2;
typedef int32_t i4;
typedef int64_t i8;

typedef float f4;
typedef double f8;

ELODIE_API struct val_str *
u2_to_str(struct mem *mem, u2 val);

#endif //CORE_COMMON_H
