#ifndef CORE_MEM_RAW_H
#define CORE_MEM_RAW_H

#include "mem.h"

struct mem_raw {
  struct mem base;
};

ELODIE_API struct mem_raw *
mem_raw_new (void);

#endif //CORE_MEM_RAW_H
