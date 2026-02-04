#pragma once

#include "bullet.h"

int spawn_bullet(Bullet *bullets, vec2 position, vec2 velocity, int lifetime);

int update_bullets(Bullet *bullets, int *active_bullets_count);
