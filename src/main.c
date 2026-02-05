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
#include "config.h"
#include "fps.h"
#include "game.h"
#include "lua_api.h"
#include "sdl.h"
#include "spritesheet.h"

#define CONFIG_FILE_PATH ("config.ini")

Bullet bullets[MAX_BULLETS_COUNT] = {0};
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

  if ((L = lua_init()) == NULL) {
    log_fatal("Failed to init Lua");
    return 1;
  }

  if (luaL_dofile(L, "./mods/base/stage2.lua") != LUA_OK) {
    log_error("Failed to load stage1.lua: %s", lua_tostring(L, -1));
    lua_pop(L, 1);
    return 1;
  }

  GameplaySection gameplay_section;
  if (load_section(L, "Stage1", &gameplay_section) != 0) {
    log_fatal("Failed to load Stage1");
    return 1;
  }

  int active_bullets = 0;

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

  while (running) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_EVENT_QUIT) {
        running = false;
      }
    }

    update_bullets(bullets, &active_bullets);

    update_section(L, &gameplay_section);

    SDL_SetRenderDrawColor(renderer, 0, 0, 0, SDL_ALPHA_OPAQUE);
    SDL_RenderClear(renderer);

    SDL_SetRenderDrawColor(renderer, 255, 255, 255, SDL_ALPHA_OPAQUE);
    render_bullets(bullets, renderer, bullets_sheet);

    SDL_RenderPresent(renderer);

    bool is_fps_updated = false;
    double fps = frame_limit(&next_frame, &is_fps_updated);
    if (is_fps_updated) {
      log_info("FPS: %lf", fps);
    }
  }

  lua_close(L);
  spritesheet_free(bullets_sheet);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return 0;
}
