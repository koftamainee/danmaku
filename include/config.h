#pragma once
#include <stdbool.h>

typedef struct {
  int window_width;
  int window_height;
  bool fullscreen;
} Configuration;

int parse_config(const char *config_file_path, Configuration *config);
