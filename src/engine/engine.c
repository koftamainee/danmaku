#include "engine.h"

#include "config/config.h"
#include "engine/bullet/bullet_system.h"
#include "engine/framerate/limiter.h"
#include "engine/render/bullet_renderer.h"
#include "engine/render/spritesheet.h"
#include "log.h"
#include "lua/lua_system.h"
#include "lua/stage.h"
#include "platform/sdl.h"

#include <assert.h>
#include <stdint.h>
#include <time.h>

#define LUA_STAGE_PATH "./scenarios/base/stages/stage1.lua"
// #define LUA_STAGE_PATH "./lua_api/bulletAPI.lua"
#define SPRITESHEET_PATH "./scenarios/base/assets/EoSD_bullets.json"
#define MAX_BULLETS_COUNT 30000

struct Engine {
  Configuration config;
  Platform *platform;
  BulletSystem *bullet_sys;
  lua_State *L;
  LuaStage *stage;
  SpriteSheet *bullets_sheet;
  FPSLimiter *fps_limiter;

  int rng_seed;

  uint64_t frames;
  uint64_t next_stats_frame;
};

Engine *engine_create(const Configuration *config) {
  assert(config != NULL);

  Engine *engine = calloc(1, sizeof(Engine));
  if (engine == NULL) {
    log_error("Failed to allocate Engine");
    return NULL;
  }

  engine->config = *config;
  engine->frames = 0;
  engine->next_stats_frame = FPS;

  engine->platform = platform_create(&engine->config);
  if (engine->platform == NULL) {
    log_error("SDL platform create failed");
    engine_destroy(engine);
    return NULL;
  }

  engine->bullet_sys = bullet_system_create(MAX_BULLETS_COUNT);
  if (engine->bullet_sys == NULL) {
    log_error("Failed to create bullet system");
    engine_destroy(engine);
    return NULL;
  }

  engine->L = lua_system_create(engine);
  if (engine->L == NULL) {
    log_error("Failed to create Lua system");
    engine_destroy(engine);
    return NULL;
  }

  engine->stage = lua_stage_create(engine->L, LUA_STAGE_PATH);
  if (engine->stage == NULL) {
    log_error("Failed to load lua stage");
    engine_destroy(engine);
    return NULL;
  }

  SDL_Renderer *renderer = platform_get_renderer(engine->platform);
  engine->bullets_sheet = spritesheet_create(renderer, SPRITESHEET_PATH);
  if (engine->bullets_sheet == NULL) {
    log_error("Failed to load bullets SpriteSheet");
    engine_destroy(engine);
    return NULL;
  }

  engine->fps_limiter = fpslimiter_create();
  if (engine->fps_limiter == NULL) {
    log_error("Failed to create fps limiter");
    engine_destroy(engine);
    return NULL;
  }

  srand((unsigned int)time(NULL));
  engine->rng_seed = rand(); // TODO: make local random function to not use
                             // default rand + srand

  return engine;
}

void engine_run(Engine *engine) {
  assert(engine != NULL);

  log_info("Main engine loop started");

  SDL_Renderer *renderer = platform_get_renderer(engine->platform);

  while (platform_is_running(engine->platform)) {
    platform_poll_events(engine->platform);

    int loops = fpslimiter_begin_frame(engine->fps_limiter);
    for (int i = 0; i < loops; i++) {
      lua_stage_update(engine->stage);

      if (lua_stage_is_finished(engine->stage)) {
        log_info("Lua stage ended. Exiting...");
        return; // This is temp
      }

      bullet_system_update(engine->bullet_sys);
      engine->frames++;
    }

    if (loops > 0) {
      platform_clear(engine->platform);
      bullet_renderer_draw(engine->bullet_sys, renderer, engine->bullets_sheet);
      platform_present(engine->platform);
    }

    fpslimiter_end_frame(engine->fps_limiter);

    if (engine->frames >= engine->next_stats_frame) {
      log_trace("Frame %lu | FPS: %.2f | Bullets: %zu | Acc: %.2fms",
                engine->frames, fpslimiter_get_fps(engine->fps_limiter),
                bullet_system_count_active(engine->bullet_sys),
                fpslimiter_get_accumulator_ms(engine->fps_limiter));

      engine->next_stats_frame += FPS;
    }
  }
}

void engine_destroy(Engine *engine) {
  if (engine == NULL) {
    return;
  }

  if (engine->bullets_sheet != NULL) {
    spritesheet_destroy(engine->bullets_sheet);
  }

  if (engine->L != NULL) {
    lua_system_destroy(engine->L);
  }

  if (engine->stage != NULL) {
    lua_stage_destroy(engine->stage);
  }

  if (engine->bullet_sys) {
    bullet_system_destroy(engine->bullet_sys);
  }

  if (engine->fps_limiter != NULL) {
    fpslimiter_destroy(engine->fps_limiter);
  }

  if (engine->platform != NULL) {
    platform_destroy(engine->platform);
  }

  free(engine);
}

BulletSystem *engine_get_bullet_system(Engine *engine) {
  assert(engine != NULL);

  return engine->bullet_sys;
}

uint64_t engine_get_time(Engine *engine) {
  assert(engine != NULL);
  return engine->frames;
}

const Configuration *engine_get_config(const Engine *engine) {
  assert(engine != NULL);
  return &engine->config;
}

FPSLimiter *engine_get_fps_limiter(Engine *engine) {
  assert(engine != NULL);
  return engine->fps_limiter;
}

int engine_get_rng_seed(const Engine *engine) {
  assert(engine != NULL);
  return engine->rng_seed;
}

LuaStage *engine_get_stage(const Engine *engine) {
  assert(engine != NULL);
  return engine->stage;
}

SpriteSheet *engine_get_bullets_sheet(Engine *engine) {
  assert(engine != NULL);
  return engine->bullets_sheet;
}
