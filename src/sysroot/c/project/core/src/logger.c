#include <stdarg.h>
#include <stdio.h>
#include <time.h>

#include "core/logger.h"

enum LogLevel {
  LOG_LEVEL_TRACE,
  LOG_LEVEL_DEBUG,
  LOG_LEVEL_INFO,
  LOG_LEVEL_WARN,
  LOG_LEVEL_FATAL
};

typedef struct {
  va_list vargs;
  char const *fmt;
  char const *filename;
  struct tm *time;
  FILE *out;
  size_t line;
  enum LogLevel level;
} LogEvent;

static void
event_init (LogEvent *ev, FILE *restrict out)
{
	if (!ev->time)
		{
			time_t t = time (NULL);
			ev->time = localtime (&t);
		}
	ev->out = out;
}

static char *level_as_str[] = {
	[LOG_LEVEL_TRACE] = "TRACE",
	[LOG_LEVEL_DEBUG] = "DEBUG",
	[LOG_LEVEL_INFO] = "INFO",
	[LOG_LEVEL_WARN] = "WARN",
	[LOG_LEVEL_FATAL] = "FATAL"
};

static char *color_of_level[] = {
	[LOG_LEVEL_TRACE] = "\x1b[94m",
	[LOG_LEVEL_DEBUG] = "\x1b[36m",
	[LOG_LEVEL_INFO] = "\x1b[32m",
	[LOG_LEVEL_WARN] = "\x1b[33m",
	[LOG_LEVEL_FATAL]="\x1b[31m"
};

static void
logger_print (LogEvent *evt)
{
	char formattedTime[18];
	formattedTime[strftime (formattedTime, sizeof (formattedTime), "%y-%m-%d %H:%M:%S", evt->time)] = '\0';

	fprintf (
		evt->out,
		"%s %s%-5s\x1b[0m \x1b[90m%s:%-2zu\x1b[0m ",
		formattedTime,
		color_of_level[evt->level], level_as_str[evt->level],
		evt->filename,
		evt->line
	);
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wformat-nonliteral"
	if (evt->level == LOG_LEVEL_FATAL)
		{
			vfprintf (evt->out, "\x1b[31m", evt->vargs);
		}
	vfprintf (evt->out, evt->fmt, evt->vargs);
#pragma GCC diagnostic pop
	fprintf (evt->out, "\n");
	fflush (evt->out);
}

void
logger_log_trace (FILE *out, char const *filename, size_t line, char const *fmt, ...)
{
	LogEvent evt = {
		.fmt   = fmt,
		.filename  = filename,
		.line  = line,
		.level = LOG_LEVEL_TRACE,
	};

	event_init (&evt, out);
	va_start(evt.vargs, fmt);
	logger_print (&evt);
	va_end(evt.vargs);
}

void
logger_log_debug (FILE *out, char const *filename, size_t line, char const *fmt, ...)
{
	LogEvent evt = {
		.fmt   = fmt,
		.filename  = filename,
		.line  = line,
		.level = LOG_LEVEL_DEBUG,
	};

	event_init (&evt, out);
	va_start(evt.vargs, fmt);
	logger_print (&evt);
	va_end(evt.vargs);
}

void
logger_log_info (FILE *out, char const *filename, size_t line, char const *fmt, ...)
{
	LogEvent evt = {
		.fmt   = fmt,
		.filename  = filename,
		.line  = line,
		.level = LOG_LEVEL_INFO,
	};

	event_init (&evt, out);
	va_start(evt.vargs, fmt);
	logger_print (&evt);
	va_end(evt.vargs);
}

void
logger_log_warn (FILE *out, char const *filename, size_t line, char const *fmt, ...)
{
	LogEvent evt = {
		.fmt   = fmt,
		.filename  = filename,
		.line  = line,
		.level = LOG_LEVEL_WARN,
	};

	event_init (&evt, out);
	va_start(evt.vargs, fmt);
	logger_print (&evt);
	va_end(evt.vargs);
}

void
logger_log_fatal (FILE *out, char const *filename, size_t line, char const *fmt, ...)
{
	LogEvent evt = {
		.fmt   = fmt,
		.filename  = filename,
		.line  = line,
		.level = LOG_LEVEL_FATAL,
	};

	event_init (&evt, out);
	va_start(evt.vargs, fmt);
	logger_print (&evt);
	va_end(evt.vargs);
}

