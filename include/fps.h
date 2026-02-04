#pragma once

#include <stdbool.h>
#include <stdint.h>

double frame_limit(uint64_t *next_frame_time, bool *fps_updated);
