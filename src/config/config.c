#include "config.h"
#include <assert.h>
#include <ini.h>
#include <log.h>
#include <memory.h>
#include <stdlib.h>
#include <string.h>

#define DEFAULT_WINDOW_WIDTH 1280
#define DEFAULT_WINDOW_HEIGHT 960

static int config_parser_handler(void *user, const char *section,
                                 const char *name, const char *value) {
  assert(user != NULL);
  assert(section != NULL);
  assert(name != NULL);
  assert(value != NULL);

  Configuration *pconfig = (Configuration *)user;

#define MATCH(s, n) strcmp(section, s) == 0 && strcmp(name, n) == 0

  if (MATCH("window", "width")) {
    pconfig->window_width = atoi(value);
  } else if (MATCH("window", "height")) {
    pconfig->window_height = atoi(value);
  } else if (MATCH("window", "fullscreen")) {
    if (strcmp(value, "true") == 0 || strcmp(value, "1") == 0) {
      pconfig->fullscreen = true;
    } else {
      pconfig->fullscreen = false;
    }
  } else {
    log_warn("Unknown ini config option");
    return 0;
  }
  return 1;
}

int parse_config(const char *config_file_path, Configuration *config) {
  assert(config != NULL);

  config->window_width = DEFAULT_WINDOW_WIDTH;
  config->window_height = DEFAULT_WINDOW_HEIGHT;
  config->fullscreen = false;

  if (ini_parse(config_file_path, config_parser_handler, config) < 0) {
    log_fatal("Failed to parse config");
    return 1;
  }

  log_info("%s config parsed", config_file_path);

  return 0;
}
