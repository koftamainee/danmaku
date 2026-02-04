#include "sdl.h"
#include "bullet.h"
#include "log.h"
#include "spritesheet.h"
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_init.h>
#include <SDL3/SDL_oldnames.h>
#include <SDL3/SDL_rect.h>
#include <SDL3/SDL_render.h>
#include <SDL3_image/SDL_image.h>

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

int render_bullets(Bullet *bullets, SDL_Renderer *renderer,
                   SpriteSheet *sprites) {
  if (bullets == NULL || renderer == NULL || sprites == NULL)
    return 1;

  SDL_Texture *texture = spritesheet_texture(sprites);

  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    if (bullets[i].lifetime == 0)
      continue;

    const SpriteRegion *region = spritesheet_get(sprites, bullets[i].sprite);

    SDL_FRect dest = {bullets[i].position[0], bullets[i].position[1],
                      region->src.w, region->src.h};

    SDL_RenderTexture(renderer, texture, &(region->src), &dest);
  }
  return 0;
}

SDL_Rect get_bullet_src_rect(int color_index) {
  SDL_Rect src;
  int offset = 32;    // each color block size
  int tile_size = 16; // bullet sprite size

  src.x = color_index * offset + (offset - tile_size) / 2;
  src.y = (offset - tile_size) / 2;
  src.w = tile_size;
  src.h = tile_size;

  return src;
}
