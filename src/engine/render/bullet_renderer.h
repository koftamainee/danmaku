#ifndef ENGINE_BULLET_RENDERER_H
#define ENGINE_BULLET_RENDERER_H

typedef struct BulletSystem BulletSystem;
typedef struct SpriteSheet SpriteSheet;
typedef struct SDL_Renderer SDL_Renderer;

void bullet_renderer_draw(BulletSystem *sys, SDL_Renderer *renderer,
                          SpriteSheet *sprites);

#endif
