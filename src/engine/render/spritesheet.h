#pragma once

#include <SDL3/SDL_rect.h>
#include <SDL3/SDL_render.h>
#include <uthash.h>

typedef struct {
  SDL_FRect src;
} SpriteRegion;

typedef struct SpriteSheet SpriteSheet;

SpriteSheet *spritesheet_create(SDL_Renderer *renderer, const char *json_path);
void spritesheet_destroy(SpriteSheet *spritesheet);

const SpriteRegion *spritesheet_get(const SpriteSheet *spritesheet,
                                    const char *name);

SDL_Texture *spritesheet_texture(const SpriteSheet *spritesheet);
