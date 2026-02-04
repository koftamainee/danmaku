#pragma once
#include <stdbool.h>

typedef struct {
  int window_width;
  int window_height;
  bool fullscreen;
} Configuration;

static int config_parser_handler(void *user, const char *section,
                                 const char *name, const char *value);

int parse_config(const char *config_file_path, Configuration *config);
