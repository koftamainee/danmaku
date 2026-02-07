// Framerate limiter to get exactly 60 FPS
//
// Reference:
// https://github.com/taisei-project/taisei/blob/master/src/framerate.c

#include "fps.h"
#include "log.h"
#include <SDL3/SDL_timer.h>
#include <string.h>

void fpscounter_reset(FPSCounter *fps) {
  const uint64_t ns_per_sec = 1000000000ULL;
  const uint64_t frametime = ns_per_sec / FPS;

  for (int i = 0; i < FPS_SAMPLE_COUNT; i++) {
    fps->frametimes[i] = frametime;
  }

  fps->fps = (double)ns_per_sec / (double)frametime;
  fps->frametime = frametime;
  fps->last_update_time = SDL_GetTicksNS();

  log_info("FPS counter reseted and ready to use");
}

void fpscounter_update(FPSCounter *fps) {
  const int log_size = FPS_SAMPLE_COUNT;
  const uint64_t ns_per_sec = 1000000000ULL;

  uint64_t update_time = SDL_GetTicksNS();
  uint64_t frametime = update_time - fps->last_update_time;

  memmove(fps->frametimes, fps->frametimes + 1,
          (size_t)(log_size - 1) * sizeof(uint64_t));
  fps->frametimes[log_size - 1] = frametime;

  uint64_t avg = 0;
  for (int i = 0; i < log_size; i++) {
    avg += fps->frametimes[i];
  }

  double avg_frametime = (double)avg / (double)log_size;
  fps->fps = (double)ns_per_sec / avg_frametime;
  fps->frametime = avg / (uint64_t)log_size;
  fps->last_update_time = update_time;
}
