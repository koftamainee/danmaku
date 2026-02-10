// Framerate limiter to get exactly 60 FPS
//
// Reference:
// https://github.com/taisei-project/taisei/blob/master/src/framerate.c

#include "limiter.h"
#include <SDL3/SDL_timer.h>
#include <assert.h>
#include <log.h>
#include <stdlib.h>
#include <string.h>

struct FPSLimiter {
  uint64_t frametimes[FPS_SAMPLE_COUNT];
  double fps;

  uint64_t accumulator;
  uint64_t frame_time_ns;
  uint64_t last_time;
  int max_frame_skip;

  int loops_this_frame;
};

FPSLimiter *fpslimiter_create(void) {
  FPSLimiter *fps = calloc(1, sizeof(FPSLimiter));
  if (fps == NULL)
    return NULL;

  fps->frame_time_ns = NS_PER_SEC / FPS;
  fps->accumulator = 0;
  fps->max_frame_skip = MAX_FRAME_SKIP;
  fps->last_time = SDL_GetTicksNS();

  for (int i = 0; i < FPS_SAMPLE_COUNT; i++) {
    fps->frametimes[i] = fps->frame_time_ns;
  }

  fps->fps = FPS;

  log_info("FPS limiter initialized");

  return fps;
}

void fpslimiter_destroy(FPSLimiter *fps) {
  free(fps);
  log_info("FPS limiter destroyed");
}

int fpslimiter_begin_frame(FPSLimiter *fps) {
  assert(fps != NULL);

  uint64_t now = SDL_GetTicksNS();
  uint64_t frame_time = now - fps->last_time;

  if (frame_time > 100000000ULL)
    frame_time = 100000000ULL;

  fps->accumulator += frame_time;
  fps->last_time = now;

  int updates = 0;
  while (fps->accumulator >= fps->frame_time_ns &&
         updates < fps->max_frame_skip) {
    fps->accumulator -= fps->frame_time_ns;
    updates++;
  }

  if (updates >= fps->max_frame_skip) {
    log_warn("Frame skip limit reached - system too slow");
    fps->accumulator = 0;
  }

  fps->loops_this_frame = updates;
  return updates;
}

void fpslimiter_end_frame(FPSLimiter *fps) {
  assert(fps != NULL);

  if (fps->accumulator < fps->frame_time_ns) {
    uint64_t sleep_time = fps->frame_time_ns - fps->accumulator;
    if (sleep_time > 500000ULL)
      SDL_DelayPrecise(sleep_time);
  }

  uint64_t now = SDL_GetTicksNS();
  uint64_t frametime = now - fps->last_time;

  memmove(fps->frametimes, fps->frametimes + 1,
          (FPS_SAMPLE_COUNT - 1) * sizeof(uint64_t));
  fps->frametimes[FPS_SAMPLE_COUNT - 1] = frametime;

  uint64_t sum = 0;
  for (int i = 0; i < FPS_SAMPLE_COUNT; i++)
    sum += fps->frametimes[i];
  double avg = (double)sum / FPS_SAMPLE_COUNT;

  fps->fps = (double)NS_PER_SEC / avg;
}

double fpslimiter_get_fps(const FPSLimiter *fps) {
  assert(fps != NULL);
  return fps->fps;
}

uint64_t fpslimiter_get_accumulator_ns(const FPSLimiter *fps) {
  assert(fps != NULL);
  return fps->accumulator;
}

double fpslimiter_get_accumulator_ms(const FPSLimiter *fps) {
  assert(fps != NULL);
  return (double)fps->accumulator / 1000000.0;
}
