// Framerate limiter to get exactly 60 FPS
//
// Reference:
// https://github.com/taisei-project/taisei/blob/master/src/framerate.c

#ifndef ENGINE_TIME_FPS_H
#define ENGINE_TIME_FPS_H

#include <stdint.h>

#define FPS 60
#define FPS_SAMPLE_COUNT 120
#define NS_PER_SEC 1000000000ULL
#define MAX_FRAME_SKIP 5
#define FRAME_TIME_NS (NS_PER_SEC / FPS)

typedef struct FPSLimiter FPSLimiter;

FPSLimiter *fpslimiter_init(void);
void fpslimiter_destroy(FPSLimiter *fps);

int fpslimiter_begin_frame(FPSLimiter *fps);
void fpslimiter_end_frame(FPSLimiter *fps);

double fpslimiter_get_fps(const FPSLimiter *fps);
uint64_t fpslimiter_get_accumulator_ns(const FPSLimiter *fps);
double fpslimiter_get_accumulator_ms(const FPSLimiter *fps);

#endif
