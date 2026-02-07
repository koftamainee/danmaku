#include "bullet.h"
#include "bullet_system.h"
#include "config.h"
#include "fps.h"
#include "lua/init.h"
#include "lua/stage.h"
#include "sdl.h"
#include "spritesheet.h"
#include <SDL3/SDL.h>
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_main.h>
#include <SDL3/SDL_pixels.h>
#include <SDL3/SDL_rect.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_timer.h>
#include <SDL3/SDL_video.h>
#include <cglm/vec2.h>
#include <lauxlib.h>
#include <log.h>
#include <lua.h>
#include <stdbool.h>
#include <stdint.h>
#include <time.h>

#define CONFIG_FILE_PATH ("./config.ini")
#define LUA_STAGE_PATH ("./mods/base/stage1.lua")
#define SPRITESHEET_PATH ("./mods/base/assets/EoSD_bullets.json")

extern size_t free_count;
extern Bullet bullets[MAX_BULLETS_COUNT];
SDL_Texture *bullet_sheet = NULL;

int main(void) {
  Configuration config = {0};
  SDL_Window *window = NULL;
  SDL_Renderer *renderer = NULL;
  lua_State *L = NULL;
  LuaStage stage = {0};
  SpriteSheet *bullets_sheet = NULL;

  FPSCounter fps_counter = {0};

  uint64_t current_time = 0;
  uint64_t accumulator = 0;
  uint64_t frames_count = 0;
  uint64_t next_stats_frame = 60;

  SDL_Event event;
  bool running = true;

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

  if (sdl_init(&config, &window, &renderer) != 0) {
    log_fatal("SDL init failed");
    return 1;
  }
  log_info("SDL system initialized");

  if ((L = lua_system_init()) == NULL) {
    log_fatal("Failed to init Lua");
    return 1;
  }
  log_info("Lua system initialized");

  if (!lua_stage_load(L, LUA_STAGE_PATH, &stage)) {
    log_fatal("Failed to load lua stage");
    return 1;
  }
  log_info("%s stage loaded", LUA_STAGE_PATH);

  bullets_sheet = spritesheet_load(renderer, SPRITESHEET_PATH);
  if (!bullets_sheet) {
    log_fatal("Failed to load bullets SpriteSheet");
    return 1;
  }
  log_info("%s spritesheet loaded", SPRITESHEET_PATH);

  bullet_system_init();
  log_info("Bullet system initialized");

  fpscounter_reset(&fps_counter);
  log_info("FPS Counter is initialized");

  current_time = SDL_GetTicksNS();

  while (running) {
    uint64_t new_time = SDL_GetTicksNS();
    uint64_t frame_time = new_time - current_time;

    if (frame_time > 100000000ULL) { // 100ms
      log_warn("Single frame took >100ms (%lu ms), clamping",
               frame_time / 1000000);
      frame_time = 100000000ULL;
    }

    current_time = new_time;
    accumulator += frame_time;

    // Handle events
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        running = false;
      }
    }

    // GAME LOGIC UPDATE - runs at EXACTLY 60 FPS
    int loops = 0;
    while (accumulator >= FRAME_TIME_NS && loops < MAX_FRAME_SKIP) {
      // Game logic
      bullet_system_update();
      lua_stage_update(&stage);
      frames_count++;

      bullet_system_compact_render_list();

      accumulator -= FRAME_TIME_NS;
      loops++;
    }

    // Handle frame skip limit
    if ((uint64_t)loops >= FRAME_TIME_NS) {
      log_warn("Frame skip limit reached - system too slow");
      accumulator = 0;
    }

    // RENDERING - only when logic updates (locks to 60 FPS)
    if (loops > 0) {
      SDL_SetRenderDrawColor(renderer, 0, 0, 0, SDL_ALPHA_OPAQUE);
      SDL_RenderClear(renderer);
      SDL_SetRenderDrawColor(renderer, 255, 255, 255, SDL_ALPHA_OPAQUE);
      render_bullets(bullets, renderer, bullets_sheet);
      SDL_RenderPresent(renderer);

      // Update FPS counter (only when we actually render)
      fpscounter_update(&fps_counter);
    }

    // Report stats every 60 logic frames (1 second of game time)
    if (frames_count >= next_stats_frame) {
      log_trace("Frame %lu | FPS: %.2f | Bullets: %ld | Acc: %.2fms",
                frames_count, fps_counter.fps, MAX_BULLETS_COUNT - free_count,
                (double)accumulator / 1000000.0);

      next_stats_frame += 60;
    }

    // Sleep if we're ahead of schedule
    if (accumulator < FRAME_TIME_NS - 1000000ULL) { // At least 1ms to spare
      uint64_t sleep_time = (FRAME_TIME_NS - accumulator) / 2;
      if (sleep_time > 500000ULL) { // >0.5ms
        SDL_DelayPrecise(sleep_time);
      }
    }
  }

  logger_stop();
  lua_close(L);
  spritesheet_free(bullets_sheet);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return 0;
}
