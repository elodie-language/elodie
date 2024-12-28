#ifndef CORE_MACRO_H
#define CORE_MACRO_H

#ifdef __cplusplus
#define ELODIE_API extern "C"
#define IS_UNIT_TEST 1
#else
#define ELODIE_API extern
#undef IS_UNIT_TEST
#endif

#define NOT_USED __attribute__((unused))

#define NO_RETURN __attribute__((noreturn))

#endif //CORE_MACRO_H
