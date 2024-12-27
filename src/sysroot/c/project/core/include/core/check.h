#ifndef CORE_MACRO_CHECK_H
#define CORE_MACRO_CHECK_H

#include <stdlib.h>
#include "logger.h"

#define ABORT(...) do{LOG_FATAL(__VA_ARGS__); abort();}while(0)
#define NOT_IMPLEMENTED_YET() do{LOG_FATAL("not implemented yet"); abort();}while(0);
#define ILLEGAL_STATE() do{LOG_FATAL("illegal state"); abort();}while(0);

#define NOP() ((void)0)

#ifdef RUN_CHECKS
#define CHECK(e)   (__builtin_expect(!(e), 0) ? ({LOG_FATAL("check '%s' failed", #e);abort();}) : NOP())
#else
#define CHECK(e) NOP()
#endif

//@formatter:off
#define CHECK_NULL(val)                     CHECK((val) == NULL)
#define CHECK_TRUE(val)                     CHECK((val) == true)
#define CHECK_FALSE(val)                    CHECK((val) == false)
#define CHECK_NOT_NULL(val)                 CHECK((val) != NULL)
#define CHECK_EQUAL(lhs, rhs)               CHECK_PRED(lhs, rhs, ==)
#define CHECK_LESS_THAN(lhs, rhs)           CHECK_PRED(lhs, rhs, <)
#define CHECK_NOT_EQUAL(lhs, rhs)           CHECK_PRED(lhs, rhs, !=)
#define CHECK_PRED(lhs, rhs, pred)          CHECK(lhs pred rhs);
#define CHECK_GREATER_THAN(lhs, rhs)        CHECK_PRED(lhs, rhs, >)
#define CHECK_LESS_THAN_EQUAL(lhs, rhs)     CHECK_PRED(lhs, rhs, <=)
#define CHECK_GREATER_THAN_EQUAL(lhs, rhs)  CHECK_PRED(lhs, rhs, >=)
//@formatter:on

#endif //CORE_MACRO_CHECK_H
