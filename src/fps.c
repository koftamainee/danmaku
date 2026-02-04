#include "fps.h"
#include <SDL3/SDL_timer.h>

double frame_limit(uint64_t *next_frame_time, bool *fps_updated) {
  const int target_fps = 60;
  const uint64_t ns_per_sec = 1000000000ULL;
  const uint64_t frame_time_ns = ns_per_sec / target_fps;

  static uint64_t last_fps_time = 0;
  static uint32_t frame_count = 0;
  static double fps = 0.0;

  uint64_t now = SDL_GetTicksNS();
  *fps_updated = 0;

  if (*next_frame_time == 0) {
    *next_frame_time = now + frame_time_ns;
    last_fps_time = now;
    return fps;
  }

  if (now < *next_frame_time) {
    SDL_DelayPrecise(*next_frame_time - now);
    *next_frame_time += frame_time_ns;
  } else {
    *next_frame_time = now + frame_time_ns;
  }

  frame_count++;

  uint64_t elapsed = now - last_fps_time;
  if (elapsed >= ns_per_sec) {
    fps = (double)frame_count * ns_per_sec / (double)elapsed;
    frame_count = 0;
    last_fps_time = now;
    *fps_updated = true;
  }

  return fps;
}
