#ifndef CORE_MEM_NULL_H
#define CORE_MEM_NULL_H

#include "mem.h"

struct mem_null {
  struct mem base;
};

ELODIE_API struct mem_null *
mem_null_new (void);

#endif // CORE_MEM_NULL_H
