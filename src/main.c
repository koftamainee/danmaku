#include "engine/bullet/bullet_system.h"
#include "engine/config/config.h"
#include "engine/render/bullet_renderer.h"
#include "engine/render/spritesheet.h"
#include "engine/time/fps.h"
#include "lua/init.h"
#include "lua/stage.h"
#include "platform/sdl.h"
#include <SDL3/SDL.h>
#include <SDL3/SDL_timer.h>
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
  FPSCounter fps_counter = {0};

  uint64_t current_time = 0;
  uint64_t accumulator = 0;
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
  log_info("%s config parsed", CONFIG_FILE_PATH);

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

  fpscounter_reset(&fps_counter);

  current_time = SDL_GetTicksNS();

  while (platform_is_running(platform)) {
    uint64_t new_time = SDL_GetTicksNS();
    uint64_t frame_time = new_time - current_time;

    if (frame_time > 100000000ULL) {
      log_warn("Single frame took >100ms (%lu ms), clamping",
               frame_time / 1000000);
      frame_time = 100000000ULL;
    }

    current_time = new_time;
    accumulator += frame_time;

    platform_poll_events(platform);

    int loops = 0;
    while (accumulator >= FRAME_TIME_NS && loops < MAX_FRAME_SKIP) {
      bullet_system_update(bullet_sys);
      lua_stage_update(&stage);
      frames_count++;

      accumulator -= FRAME_TIME_NS;
      loops++;
    }

    if ((uint64_t)loops >= MAX_FRAME_SKIP) {
      log_warn("Frame skip limit reached - system too slow");
      accumulator = 0;
    }

    if (loops > 0) {
      platform_clear(platform);
      bullet_renderer_draw(bullet_sys, renderer, bullets_sheet);
      platform_present(platform);

      fpscounter_update(&fps_counter);
    }

    if (frames_count >= next_stats_frame) {
      log_trace("Frame %lu | FPS: %.2f | Bullets: %zu | Acc: %.2fms",
                frames_count, fps_counter.fps,
                bullet_system_count_active(bullet_sys),
                (double)accumulator / 1000000.0);

      next_stats_frame += 60;
    }

    if (accumulator < FRAME_TIME_NS - 1000000ULL) {
      uint64_t sleep_time = (FRAME_TIME_NS - accumulator) / 2;
      if (sleep_time > 500000ULL) {
        SDL_DelayPrecise(sleep_time);
      }
    }
  }

  logger_stop();
  spritesheet_destroy(bullets_sheet);
  lua_system_destroy(L);
  bullet_system_destroy(bullet_sys);
  platform_destroy(platform);

  return 0;
}
