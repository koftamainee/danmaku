#pragma once

#include "bullet.h"
#include "config.h"
#include "spritesheet.h"
#include <SDL3/SDL_render.h>

int sdl_init(const Configuration *config, SDL_Window **window,
             SDL_Renderer **renderer);

int render_bullets(Bullet *bullets, SDL_Renderer *renderer,
                   SpriteSheet *sprites);

SDL_Rect get_bullet_src_rect(int color_index);
