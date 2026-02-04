#pragma once

#include <SDL3/SDL_rect.h>
#include <SDL3/SDL_render.h>
#include <uthash.h>

typedef struct {
  SDL_FRect src;
} SpriteRegion;

typedef struct SpriteEntry {
  char *name;
  SpriteRegion region;
  UT_hash_handle hh;
} SpriteEntry;

typedef struct SpriteSheet SpriteSheet;

SpriteSheet *spritesheet_load(SDL_Renderer *renderer, const char *json_path);

const SpriteRegion *spritesheet_get(const SpriteSheet *spritesheet,
                                    const char *name);

SDL_Texture *spritesheet_texture(const SpriteSheet *spritesheet);

void spritesheet_free(SpriteSheet *spritesheet);
