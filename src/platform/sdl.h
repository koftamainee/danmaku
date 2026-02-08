#ifndef PLATFORM_SDL_H
#define PLATFORM_SDL_H

#include "config/config.h"
#include <stdbool.h>

typedef struct SDL_Window SDL_Window;
typedef struct SDL_Renderer SDL_Renderer;

typedef struct Platform {
  SDL_Window *window;
  SDL_Renderer *renderer;
  bool running;
} Platform;

Platform *platform_init(const Configuration *config);
void platform_destroy(Platform *platform);

bool platform_is_running(const Platform *platform);
void platform_poll_events(Platform *platform);

SDL_Renderer *platform_get_renderer(Platform *platform);
void platform_clear(Platform *platform);
void platform_present(Platform *platform);

#endif
