// Slightly modified version of
// https://github.com/koftamainee/fuse/blob/master/include/src/logger.c

#include "log.h"
#include <stdlib.h>
#include <time.h>

#define MAX_LOGGERS 16

typedef struct {
  FILE *fp;
  log_level level;
} Logger;

static struct {
  log_level level;
  Logger loggers[MAX_LOGGERS];
  size_t loggers_count;
} L = {LOG_INFO, {{NULL, LOG_INFO}}, 0};

static const char *level_string[] = {"TRACE", "DEBUG", "INFO",
                                     "WARN",  "ERROR", "FATAL"};
static const char *level_color[] = {"\x1b[36m", "\x1b[34m", "\x1b[32m",
                                    "\x1b[33m", "\x1b[31m", "\x1b[41m"};
#define COLOR_RESET "\x1b[0m"

static void log_to_stream(FILE *stream, log_level level, const char *file,
                          int line, const char *fmt, va_list ap) {
  char time_buf[16];
  time_t t = time(NULL);
  strftime(time_buf, sizeof(time_buf), "%H:%M:%S", localtime(&t));

  if (stream == stdout || stream == stderr) {
    fprintf(stream, "%s %s%-5s%s %s:%d: ", time_buf, level_color[level],
            level_string[level], COLOR_RESET, file, line);
  } else {
    fprintf(stream, "%s %-5s %s:%d: ", time_buf, level_string[level], file,
            line);
  }

  vfprintf(stream, fmt, ap);
  fprintf(stream, "\n");
}

void log_set_level(log_level level) { L.level = level; }

int log_add_fp(FILE *fp, log_level level) {
  if (!fp) {
    fprintf(stderr, "Error: Passed file pointer is NULL.\n");
    return 1;
  }
  if (L.loggers_count >= MAX_LOGGERS) {
    fprintf(stderr, "Error: Reached max logger count.\n");
    return 1;
  }
  L.loggers[L.loggers_count++] = (Logger){fp, level};
  return EXIT_SUCCESS;
}

void log_log(log_level level, const char *file, int line, const char *fmt,
             ...) {
  if (!fmt) {
    fprintf(stderr, "Warning: fmt passed to log is NULL.\n");
    return;
  }
  if (level < L.level)
    return;

  va_list ap, ap_cpy;
  va_start(ap, fmt);
  for (size_t i = 0; i < L.loggers_count; ++i) {
    if (level >= L.loggers[i].level) {
      va_copy(ap_cpy, ap);
      log_to_stream(L.loggers[i].fp, level, file, line, fmt, ap_cpy);
      va_end(ap_cpy);
    }
  }
  va_end(ap);
}

void vlog_log(log_level level, const char *file, int line, const char *fmt,
              va_list ap) {
  if (!fmt) {
    fprintf(stderr, "Warning: fmt passed to log is NULL.\n");
    return;
  }
  if (level < L.level)
    return;

  va_list ap_cpy;
  for (size_t i = 0; i < L.loggers_count; ++i) {
    if (level >= L.loggers[i].level) {
      va_copy(ap_cpy, ap);
      log_to_stream(L.loggers[i].fp, level, file, line, fmt, ap_cpy);
      va_end(ap_cpy);
    }
  }
}

int logger_start(void) { return log_add_fp(stderr, LOG_TRACE); }

void logger_stop(void) {
  for (size_t i = 0; i < L.loggers_count; ++i) {
    FILE *f = L.loggers[i].fp;
    if (f != stdout && f != stderr && f)
      fclose(f);
  }
}
