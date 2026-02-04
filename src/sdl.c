#include "sdl.h"
#include "bullet.h"
#include "log.h"
#include <SDL3/SDL_init.h>
#include <SDL3/SDL_oldnames.h>
#include <SDL3/SDL_render.h>

int sdl_init(const Configuration *config, SDL_Window **window,
             SDL_Renderer **renderer) {
  if (config == NULL) {
    return 1;
  }

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    log_fatal("SDL_Init Error: %s", SDL_GetError());
    return 1;
  }

  log_info("SDL is initilized");

  bool is_window_and_renderer_created = SDL_CreateWindowAndRenderer(
      "danmaku", config->window_width, config->window_height,
      SDL_WINDOW_RESIZABLE, window, renderer);

  if (!is_window_and_renderer_created) {
    log_fatal("SDL_CreateWindowAndRenderer Error: %s", SDL_GetError());
    return 1;
  }

  SDL_SetRenderLogicalPresentation(*renderer, 640, 480,
                                   SDL_LOGICAL_PRESENTATION_LETTERBOX);

  log_info("SDL3 window and renderer initialized");

  if (config->fullscreen) {
    SDL_SetWindowFullscreen(*window, true);
    log_info("Fullscreen mode set to true");
  } else {

    SDL_SetWindowFullscreen(*window, false);
    log_info("Fullscreen mode set to false");
  }

  return 0;
}

int render_bullets(Bullet *bullets, SDL_Renderer *renderer) {
  if (bullets == NULL || renderer == NULL)
    return 1;

  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    if (bullets[i].lifetime == 0)
      continue;

    SDL_FRect r = {
        .x = bullets[i].position[0],
        .y = bullets[i].position[1],
        .w = 3.0f,
        .h = 3.0f,
    };
    SDL_RenderFillRect(renderer, &r);
  }
  return 0;
}
