#include <SDL3/SDL_timer.h>

#include <SDL3/SDL.h>
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_main.h>
#include <SDL3/SDL_pixels.h>
#include <SDL3/SDL_rect.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>

#include <cglm/vec2.h>
#include <lauxlib.h>
#include <log.h>
#include <lua.h>
#include <stdbool.h>

#include <stdint.h>
#include <time.h>

#include "bullet.h"
#include "bullet_system.h"
#include "config.h"
#include "fps.h"
#include "lua/init.h"
#include "lua/stage.h"
#include "lua_api.h"
#include "sdl.h"
#include "spritesheet.h"

#define CONFIG_FILE_PATH ("./config.ini")

extern size_t free_count;

extern Bullet bullets[MAX_BULLETS_COUNT];
SDL_Texture *bullet_sheet = NULL;

int main(void) {
  Configuration config;
  if (parse_config(CONFIG_FILE_PATH, &config) != 0) {
    log_fatal("Failed to parse config");
    return 1;
  }

  SDL_Window *window = NULL;
  SDL_Renderer *renderer = NULL;

  if (sdl_init(&config, &window, &renderer) != 0) {
    log_fatal("SDL init failed");
    return 1;
  }

  lua_State *L = NULL;

  if ((L = lua_system_init()) == NULL) {
    log_fatal("Failed to init Lua");
    return 1;
  }

  log_info("Lua system initialized");

  LuaStage stage;
  if (!lua_stage_load(L, "./mods/base/stage1.lua", &stage)) {
    log_fatal("Failed to load lua stage");
    return 1;
  }

  log_info("Lua Stage loaded");

  SDL_Event event;
  bool running = true;

  uint64_t next_frame = 0;

  SpriteSheet *bullets_sheet =
      spritesheet_load(renderer, "./mods/base/assets/EoSD_bullets.json");
  if (bullets_sheet == NULL) {
    log_fatal("Failed to load bullets SpriteSheet");
    return 1;
  }

  log_info("SpriteSheet with bullets loaded");

  bullet_system_init();

  log_info("Bullet system initialized");

  size_t frames = 0;

  while (running) {
    frames++;
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        running = false;
      }
    }

    bullet_system_update();

    lua_stage_update(&stage);

    SDL_SetRenderDrawColor(renderer, 0, 0, 0, SDL_ALPHA_OPAQUE);
    SDL_RenderClear(renderer);

    SDL_SetRenderDrawColor(renderer, 255, 255, 255, SDL_ALPHA_OPAQUE);

    if (frames % 120 == 0) {
      bullet_system_compact_render_list();
    }

    render_bullets(bullets, renderer, bullets_sheet);

    SDL_RenderPresent(renderer);

    bool is_fps_updated = false;
    double fps = frame_limit(&next_frame, &is_fps_updated);
    if (is_fps_updated) {
      log_info("FPS: %lf, bullets: %ld", fps, MAX_BULLETS_COUNT - free_count);
    }
  }

  lua_close(L);
  spritesheet_free(bullets_sheet);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return 0;
}
