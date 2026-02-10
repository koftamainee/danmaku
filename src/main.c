#include "config/config.h"
#include <log.h>
#include <stdbool.h>
#include <stdint.h>

#include "engine/engine.h"

#define CONFIG_FILE_PATH "./config.ini"

int main(void) {
  Configuration config = {0};

  if (logger_start() != 0) {
    fprintf(stderr, "WARN: Failed to start logger\n");
  }

  log_set_level(LOG_TRACE);
  log_info("Logger is initialized");

  if (parse_config(CONFIG_FILE_PATH, &config) != 0) {
    log_fatal("Failed to parse config");
    return 1;
  }

  Engine *engine = engine_create(&config);
  if (engine == NULL) {
    log_fatal("Fatal error occured while creating engine");
    return 1;
  }

  engine_run(engine);
  engine_destroy(engine);

  logger_stop();
  return 0;
}
