#ifndef ENGINE_ENGINE_H
#define ENGINE_ENGINE_H

#include "config/config.h"
#include "engine/bullet/bullet_system.h"
#include "engine/framerate/limiter.h"
#include "engine/render/spritesheet.h"
#include "lua/stage.h"
#include <stdint.h>

typedef struct Engine Engine;

Engine *engine_create(const Configuration *config);
void engine_run(Engine *engine);
void engine_destroy(Engine *engine);

const Configuration *engine_get_config(const Engine *engine);
BulletSystem *engine_get_bullet_system(Engine *engine);
FPSLimiter *engine_get_fps_limiter(Engine *engine);

uint64_t engine_get_time(Engine *engine);
int engine_get_rng_seed(const Engine *engine);

LuaStage *engine_get_stage(const Engine *engine);
SpriteSheet *engine_get_bullets_sheet(Engine *engine);

#endif // !ENGINE_ENGINE_H
