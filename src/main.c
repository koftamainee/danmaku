#include "config/config.h"
#include "engine/bullet/bullet_system.h"
#include "engine/framerate/limiter.h"
#include "engine/render/bullet_renderer.h"
#include "engine/render/spritesheet.h"
#include "lua/init.h"
#include "lua/stage.h"
#include "platform/sdl.h"
#include <log.h>
#include <stdbool.h>
#include <stdint.h>

#define CONFIG_FILE_PATH "./config.ini"
#define LUA_STAGE_PATH "./scenarios/base/stages/stage1.lua"
#define SPRITESHEET_PATH "./scenarios/base/assets/EoSD_bullets.json"
#define MAX_BULLETS_COUNT 30000

int main(void) {
  Configuration config = {0};
  Platform *platform = NULL;
  BulletSystem *bullet_sys = NULL;
  lua_State *L = NULL;
  LuaStage stage = {0};
  SpriteSheet *bullets_sheet = NULL;
  FPSLimiter *fps_limiter = NULL;

  uint64_t frames_count = 0;
  uint64_t next_stats_frame = 60;

  if (logger_start() != 0) {
    fprintf(stderr, "WARN: Failed to start logger\n");
  }
  log_set_level(LOG_TRACE);
  log_info("Logger is initialized");

  if (parse_config(CONFIG_FILE_PATH, &config) != 0) {
    log_fatal("Failed to parse config");
    return 1;
  }

  platform = platform_init(&config);
  if (platform == NULL) {
    log_fatal("SDL platform init failed");
    return 1;
  }

  bullet_sys = bullet_system_init(MAX_BULLETS_COUNT);
  if (bullet_sys == NULL) {
    log_fatal("Failed to create bullet system");
    platform_destroy(platform);
    return 1;
  }

  L = lua_system_init(bullet_sys);
  if (L == NULL) {
    log_fatal("Failed to init Lua");
    bullet_system_destroy(bullet_sys);
    platform_destroy(platform);
    return 1;
  }

  if (!lua_stage_load(L, LUA_STAGE_PATH, &stage)) {
    log_fatal("Failed to load lua stage");
    lua_system_destroy(L);
    bullet_system_destroy(bullet_sys);
    platform_destroy(platform);
    return 1;
  }

  SDL_Renderer *renderer = platform_get_renderer(platform);
  bullets_sheet = spritesheet_load(renderer, SPRITESHEET_PATH);
  if (bullets_sheet == NULL) {
    log_fatal("Failed to load bullets SpriteSheet");
    lua_system_destroy(L);
    bullet_system_destroy(bullet_sys);
    platform_destroy(platform);
    return 1;
  }

  fps_limiter = fpslimiter_init();
  if (fps_limiter == NULL) {
    log_fatal("Failed to init fps limiter");
    return 1;
  }

  while (platform_is_running(platform)) {
    platform_poll_events(platform);

    int loops = fpslimiter_begin_frame(fps_limiter);
    for (int i = 0; i < loops; i++) {
      lua_stage_update(&stage);
      bullet_system_update(bullet_sys);
      frames_count++;
    }

    if (loops > 0) {
      platform_clear(platform);
      bullet_renderer_draw(bullet_sys, renderer, bullets_sheet);
      platform_present(platform);
    }

    fpslimiter_end_frame(fps_limiter);

    if (frames_count >= next_stats_frame) {
      log_trace("Frame %lu | FPS: %.2f | Bullets: %zu | Acc: %.2fms",
                frames_count, fpslimiter_get_fps(fps_limiter),
                bullet_system_count_active(bullet_sys),
                fpslimiter_get_accumulator_ms(fps_limiter));

      next_stats_frame += FPS;
    }
  }

  logger_stop();
  spritesheet_destroy(bullets_sheet);
  lua_system_destroy(L);
  bullet_system_destroy(bullet_sys);
  fpslimiter_destroy(fps_limiter);
  platform_destroy(platform);

  return 0;
}
