#ifndef CORE_LOGGER_H
#define CORE_LOGGER_H

#include "stdio.h"
#include "core.h"
#include "util.h"

HAMAL_API void
logger_log_trace (FILE *out, char const *filename, size_t line, char const *fmt, ...);

HAMAL_API void
logger_log_debug (FILE *out, char const *filename, size_t line, char const *fmt, ...);

HAMAL_API void
logger_log_info (FILE *out, char const *filename, size_t line, char const *fmt, ...);

HAMAL_API void
logger_log_warn (FILE *out, char const *filename, size_t line, char const *fmt, ...);

HAMAL_API void
logger_log_fatal (FILE *out, char const *filename, size_t line, char const *fmt, ...);

#define LOG_LEVEL_SILENT_ 0
#define LOG_LEVEL_TRACE_ 1
#define LOG_LEVEL_DEBUG_ 2
#define LOG_LEVEL_INFO_ 3
#define LOG_LEVEL_WARN_ 4
#define LOG_LEVEL_FATAL_ 5

#if LOG_LEVEL > 0

#if LOG_LEVEL <= LOG_LEVEL_TRACE_
#define LOG_TRACE(...)  logger_log_trace(stdout, __FILE_NAME__, __LINE__, __VA_ARGS__)
#else
#define LOG_TRACE(...)
#endif

#if LOG_LEVEL <= LOG_LEVEL_DEBUG_
#define LOG_DEBUG(...) logger_log_debug(stdout, __FILE_NAME__, __LINE__, __VA_ARGS__)
#else
#define LOG_DEBUG(...)
#endif

#if LOG_LEVEL <= LOG_LEVEL_INFO_
#define LOG_INFO(...)  logger_log_info(stdout, __FILE_NAME__, __LINE__, __VA_ARGS__)
#else
#define LOG_INFO(...)
#endif

#if LOG_LEVEL <= LOG_LEVEL_WARN_
#define LOG_WARN(...)  logger_log_warn(stdout, __FILE_NAME__, __LINE__, __VA_ARGS__)
#else
#define LOG_WARN(...)
#endif

#if LOG_LEVEL <= LOG_LEVEL_FATAL_
#define LOG_FATAL(...) logger_log_fatal(stdout, __FILE_NAME__, __LINE__, __VA_ARGS__)
#else
#define LOG_FATAL(...)
#endif

#else
#define LOG_TRACE(...)
#define LOG_DEBUG(...)
#define LOG_INFO(...)
#define LOG_WARN(...)
#define LOG_FATAL(...)
#endif

#undef LOG_LEVEL_TRACE_
#undef LOG_LEVEL_DEBUG_
#undef LOG_LEVEL_INFO_
#undef LOG_LEVEL_WARN_
#undef LOG_LEVEL_FATAL_
#undef LOG_LEVEL_SILENT_

#endif //CORE_LOGGER_H
