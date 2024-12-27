#ifndef CORE_ALGO_BIT_H
#define CORE_ALGO_BIT_H

#include "core/macro.h"
#include "core/core.h"

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[bit 8 bit]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct bit8 {
  u1 data;
};

HAMAL_API void
bit8_init (struct bit8 *self);

HAMAL_API void
bit8_set (struct bit8 *self, size_t idx);

HAMAL_API void
bit8_set_mask (struct bit8 *self, u1 mask);

HAMAL_API void
bit8_unset (struct bit8 *self, size_t idx);

HAMAL_API void
bit8_unset_mask (struct bit8 *self, u1 mask);

HAMAL_API void
bit8_toggle (struct bit8 *self, size_t idx);

HAMAL_API void
bit8_toggle_mask (struct bit8 *self, u1 mask);

HAMAL_API bool
bit8_at (struct bit8 *self, size_t idx);

HAMAL_API u1
bit8_get_mask (struct bit8 *self);

HAMAL_API void
bit8_reset (struct bit8 *self);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[bit 64 bit]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
struct bit64 {
  union {
	struct bit8 bucket[8];
	u8 numeric;
  };
};

HAMAL_API void
bit64_init (struct bit64 *self);

HAMAL_API void
bit64_set (struct bit64 *self, size_t idx);

HAMAL_API void
bit64_set_mask (struct bit64 *self, u8 mask);

HAMAL_API void
bit64_unset (struct bit64 *self, size_t idx);

HAMAL_API void
bit64_unset_mask (struct bit64 *self, u8 mask);

HAMAL_API void
bit64_toggle (struct bit64 *self, size_t idx);

HAMAL_API void
bit64_toggle_mask (struct bit64 *self, u8 mask);

HAMAL_API bool
bit64_at (struct bit64 *self, size_t idx);

HAMAL_API u8
bit64_get_mask (struct bit64 *self);

HAMAL_API void
bit64_reset (struct bit64 *self);

#endif //CORE_ALGO_BIT_H
