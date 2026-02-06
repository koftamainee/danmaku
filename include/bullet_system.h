#pragma once
#include "bullet.h"

#define MAX_BULLETS_COUNT (30000)

void bullet_system_init(void);

BulletID spawn_bullet(const Bullet *init);

void kill_bullet(BulletID id);

void bullet_system_update(void);

Bullet *bullet_get(BulletID id);

bool bullet_is_alive(BulletID id);

void bullet_system_compact_render_list(void);
