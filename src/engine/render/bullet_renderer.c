#include "bullet_renderer.h"
#include "engine/bullet/bullet.h"
#include "engine/bullet/bullet_system.h"
#include "spritesheet.h"

#include <SDL3/SDL_render.h>
#include <log.h>

struct RenderContext {
  SDL_Renderer *renderer;
  SpriteSheet *sprites;
  SDL_Texture *texture;
};

static void render_bullet_callback(const Bullet *b, void *user_data) {
  assert(b != NULL);
  assert(user_data != NULL);

  struct RenderContext *ctx = user_data;

  const SpriteRegion *region = spritesheet_get(ctx->sprites, b->sprite);
  if (region == NULL) {
    log_error("Invalid sprite name: %s", b->sprite);
    return;
  }

  SDL_FRect dest = {.x = b->position[0],
                    .y = b->position[1],
                    .w = region->src.w,
                    .h = region->src.h};

  SDL_RenderTexture(ctx->renderer, ctx->texture, &region->src, &dest);
}

void bullet_renderer_draw(BulletSystem *sys, SDL_Renderer *renderer,
                          SpriteSheet *sprites) {
  assert(sys != NULL);
  assert(renderer != NULL);
  assert(sprites != NULL);

  struct RenderContext ctx = {.renderer = renderer,
                              .sprites = sprites,
                              .texture = spritesheet_texture(sprites)};

  bullet_system_foreach_render_order(sys, render_bullet_callback, &ctx);
}
