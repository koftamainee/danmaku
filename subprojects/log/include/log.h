// Slightly modified version of
// https://github.com/koftamainee/fuse/blob/master/include/src/logger.h

#ifndef LOG_H
#define LOG_H

#include <stdarg.h>
#include <stdio.h>

typedef enum {
  LOG_TRACE,
  LOG_DEBUG,
  LOG_INFO,
  LOG_WARN,
  LOG_ERROR,
  LOG_FATAL
} log_level;

int logger_start(void);
void logger_stop(void);
void log_set_level(log_level level);
int log_add_fp(FILE *fp, log_level level);

void log_log(log_level level, const char *file, int line, const char *fmt, ...);
void vlog_log(log_level level, const char *file, int line, const char *fmt,
              va_list ap);

#ifndef NDEBUG

#define log_trace(...) log_log(LOG_TRACE, __FILE__, __LINE__, __VA_ARGS__)
#define log_debug(...) log_log(LOG_DEBUG, __FILE__, __LINE__, __VA_ARGS__)

#else

#define log_trace(...) ((void)0)
#define log_debug(...) ((void)0)

#endif

#define log_info(...) log_log(LOG_INFO, __FILE__, __LINE__, __VA_ARGS__)
#define log_warn(...) log_log(LOG_WARN, __FILE__, __LINE__, __VA_ARGS__)
#define log_error(...) log_log(LOG_ERROR, __FILE__, __LINE__, __VA_ARGS__)
#define log_fatal(...) log_log(LOG_FATAL, __FILE__, __LINE__, __VA_ARGS__)

#endif
