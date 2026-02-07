#include "sdl.h"
#include <SDL3/SDL.h>
#include <SDL3/SDL_error.h>
#include <SDL3/SDL_events.h>
#include <SDL3/SDL_init.h>
#include <SDL3/SDL_render.h>
#include <SDL3/SDL_video.h>
#include <log.h>
#include <stdlib.h>

#define GAME_WIDTH 640
#define GAME_HEIGHT 480

Platform *platform_init(const Configuration *config) {
  if (config == NULL) {
    log_error("Cannot init platform: config is NULL");
    return NULL;
  }

  Platform *platform = calloc(1, sizeof(Platform));
  if (!platform) {
    log_error("Failed to allocate platform");
    return NULL;
  }

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    log_fatal("SDL_Init Error: %s", SDL_GetError());
    free(platform);
    return NULL;
  }

  bool success = SDL_CreateWindowAndRenderer(
      "danmaku", config->window_width, config->window_height,
      SDL_WINDOW_RESIZABLE, &platform->window, &platform->renderer);

  if (!success) {
    log_fatal("SDL_CreateWindowAndRenderer Error: %s", SDL_GetError());
    SDL_Quit();
    free(platform);
    return NULL;
  }

  log_info("SDL window and renderer created. Window size is set to %dx%d",
           config->window_width, config->window_height);

  SDL_SetRenderLogicalPresentation(platform->renderer, GAME_WIDTH, GAME_HEIGHT,
                                   SDL_LOGICAL_PRESENTATION_LETTERBOX);

  log_info("Game native resolution is set to %dx%d", GAME_WIDTH, GAME_HEIGHT);

  if (config->fullscreen) {
    SDL_SetWindowFullscreen(platform->window, true);
    log_info("Fullscreen mode set to true");
  } else {
    SDL_SetWindowFullscreen(platform->window, false);
    log_info("Fullscreen mode set to false");
  }

  platform->running = true;

  log_info("SDL platform is initialized");

  return platform;
}

void platform_destroy(Platform *platform) {
  if (platform == NULL) {
    return;
  }

  if (platform->renderer != NULL) {
    SDL_DestroyRenderer(platform->renderer);
  }
  if (platform->window != NULL) {
    SDL_DestroyWindow(platform->window);
  }

  SDL_Quit();
  free(platform);

  log_info("SDL platform destroyed");
}

bool platform_is_running(const Platform *platform) {
  return platform != NULL && platform->running;
}

void platform_poll_events(Platform *platform) {
  if (platform == NULL) {
    return;
  }

  SDL_Event event;
  while (SDL_PollEvent(&event)) {
    if (event.type == SDL_EVENT_QUIT) {
      platform->running = false;
    }
  }
}

SDL_Renderer *platform_get_renderer(Platform *platform) {
  return platform != NULL ? platform->renderer : NULL;
}

void platform_clear(Platform *platform) {
  if (platform == NULL || platform->renderer == NULL) {
    return;
  }

  SDL_SetRenderDrawColor(platform->renderer, 0, 0, 0, 255);
  SDL_RenderClear(platform->renderer);
}

void platform_present(Platform *platform) {
  if (platform == NULL || platform->renderer == NULL) {
    return;
  }

  SDL_RenderPresent(platform->renderer);
}
