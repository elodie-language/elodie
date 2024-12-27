#include "stdio.h"
#include "core/algo/algo-bit.h"
#include "core/check.h"

typedef struct bit8 b1;
typedef struct bit64 b8;

void
bit8_init (b1 *self)
{
	CHECK_NOT_NULL(self);
	self->data = 0;
}

void
bit8_set (b1 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 8);
	self->data |= 1UL << idx;
}

void
bit8_set_mask (b1 *self, u1 mask)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < 8; idx++)
		{
			if ((mask >> idx) & 1U)
				{
					bit8_set (self, idx);
				}
		}
}

void
bit8_unset (b1 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 8);
	self->data &= ~(1UL << idx);
}

void
bit8_unset_mask (b1 *self, u1 mask)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < 8; idx++)
		{
			if ((mask >> idx) & 1U)
				{
					bit8_unset (self, idx);
				}
		}
}

void
bit8_toggle (b1 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 8);
	self->data ^= 1UL << idx;
}

void
bit8_toggle_mask (b1 *self, u1 mask)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < 8; idx++)
		{
			if ((mask >> idx) & 1U)
				{
					bit8_toggle (self, idx);
				}
		}
}

bool
bit8_at (b1 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 8);
	return (self->data >> idx) & 1U;
}

u1
bit8_get_mask (b1 *self)
{
	CHECK_NOT_NULL(self);
	return self->data;
}

void
bit8_reset (b1 *self)
{
	CHECK_NOT_NULL(self);
	bit8_init (self);
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[bit 64 bit]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
inline static b1 *
bit64_find_bit (b8 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	return &self->bucket[idx / 8];
}

inline static size_t
bit64_resolve_index (size_t idx)
{
	return idx % 8;
}

void
bit64_init (b8 *self)
{
	CHECK_NOT_NULL(self);
	self->numeric = 0;
}

void
bit64_set (b8 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 64);
	bit8_set (bit64_find_bit (self, idx), bit64_resolve_index (idx));
}

void
bit64_set_mask (b8 *self, u8 mask)
{
	CHECK_NOT_NULL(self);
	u1 *mask_ptr = (u1 *)&mask;
	for (size_t idx = 0; idx < 8; idx++)
		{
			bit8_set_mask (&self->bucket[idx], mask_ptr[idx]);
		}
}

void
bit64_unset (b8 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 64);
	bit8_unset (bit64_find_bit (self, idx), bit64_resolve_index (idx));
}

void
bit64_unset_mask (b8 *self, u8 mask)
{
	CHECK_NOT_NULL(self);
	u1 *mask_ptr = (u1 *)&mask;
	for (size_t idx = 0; idx < 8; idx++)
		{
			bit8_unset_mask (&self->bucket[idx], mask_ptr[idx]);
		}
}

void
bit64_toggle (b8 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 64);
	bit8_toggle (bit64_find_bit (self, idx), bit64_resolve_index (idx));
}

void
bit64_toggle_mask (b8 *self, u8 mask)
{
	CHECK_NOT_NULL(self);
	u1 *mask_ptr = (u1 *)&mask;
	for (size_t idx = 0; idx < 8; idx++)
		{
			bit8_toggle_mask (&self->bucket[idx], mask_ptr[idx]);
		}
}

bool
bit64_at (b8 *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, 64);
	return bit8_at (bit64_find_bit (self, idx), bit64_resolve_index (idx));
}

u8
bit64_get_mask (b8 *self)
{
	CHECK_NOT_NULL(self);
	return self->numeric;
}

void
bit64_reset (b8 *self)
{
	CHECK_NOT_NULL(self);
	self->numeric = 0;
}
