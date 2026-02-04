#pragma once

#include "bullet.h"

int spawn_bullet(Bullet *bullets, vec2 position, vec2 velocity, int lifetime,
                 const char *sprite);

int update_bullets(Bullet *bullets, int *active_bullets_count);
