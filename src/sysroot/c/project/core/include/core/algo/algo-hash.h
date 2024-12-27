#ifndef CORE_ALGO_HASH_H
#define CORE_ALGO_HASH_H

#include "core/mem/mem.h"
#include "core/bytes/bytes-view.h"

struct hash4 {
  u4 value;
};

HAMAL_API bool
hash4_equal (struct hash4 lhs, struct hash4 rhs);

struct hash8 {
  u8 value;
};

HAMAL_API bool
hash8_equal (struct hash8 lhs, struct hash8 rhs);

struct hash8_fn;
struct hash4_fn;

typedef struct hash8 (*hash8_fn_ptr) (struct hash8_fn *self, struct bytes_view bytes);
typedef struct hash4 (*hash4_fn_ptr) (struct hash4_fn *self, struct bytes_view bytes);

struct hash8_fn {
  u8 seed_1;
  u8 seed_2;
  hash8_fn_ptr fn;
};

struct hash4_fn {
  hash4_fn_ptr fn;
};

HAMAL_API struct hash8_fn
hash_fn_sip_8 (u8 seed_1, u8 seed_2);

HAMAL_API struct hash8_fn
hash_fn_murmur_3 (u8 seed);

HAMAL_API struct hash8_fn
hash_fn_identity_8 (void);

HAMAL_API struct hash4_fn
hash_fn_crc4 (void);

HAMAL_API struct hash8
hash8_of (struct hash8_fn self, struct bytes_view bytes);

HAMAL_API struct hash4
hash4_of (struct hash4_fn self, struct bytes_view bytes);

HAMAL_API struct hash8
hash8_of_hashes (struct hash8_fn self, struct hash8 *arr, size_t count);

HAMAL_API struct hash4
hash4_of_hashes (struct hash4_fn self, struct hash4 *arr, size_t count);

#endif //CORE_ALGO_HASH_H
