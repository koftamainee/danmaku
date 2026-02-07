// Framerate limiter to get exactly 60 FPS
//
// Reference:
// https://github.com/taisei-project/taisei/blob/master/src/framerate.h

#pragma once

#include <stdint.h>

#define FPS (60)
#define FPS_SAMPLE_COUNT (120)
#define NS_PER_SEC (1000000000ULL)
#define FRAME_TIME_NS (NS_PER_SEC / FPS)
#define MAX_FRAME_SKIP (5)

typedef struct {
  uint64_t frametimes[FPS_SAMPLE_COUNT]; // Rolling window of frame times
  double fps;                            // Average FPS over last N frames
  uint64_t frametime;                    // Average frame time in nanoseconds
  uint64_t last_update_time;             // Last time counter was updated
} FPSCounter;

void fpscounter_reset(FPSCounter *fps);

void fpscounter_update(FPSCounter *fps);
