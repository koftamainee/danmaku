#ifndef ENGINE_BULLET_SYSTEM_H
#define ENGINE_BULLET_SYSTEM_H

#include "bullet.h"
#include "bullet_id.h"
#include <stdbool.h>
#include <stddef.h>

typedef struct BulletSystem BulletSystem;

typedef void (*BulletIteratorFn)(const Bullet *bullet, void *user_data);

BulletSystem *bullet_system_init(size_t capacity);
void bullet_system_destroy(BulletSystem *system);

BulletID bullet_system_spawn(BulletSystem *system, const Bullet *init);
void bullet_system_kill(BulletSystem *system, BulletID id);
Bullet *bullet_system_get(BulletSystem *system, BulletID id);
bool bullet_system_is_alive(BulletSystem *system, BulletID id);

void bullet_system_update(BulletSystem *system);
void bullet_system_clear(BulletSystem *system);

void bullet_system_foreach(BulletSystem *system, BulletIteratorFn callback,
                           void *user_data);
void bullet_system_foreach_render_order(BulletSystem *system,
                                        BulletIteratorFn callback,
                                        void *user_data);

size_t bullet_system_count_active(const BulletSystem *system);
size_t bullet_system_count_free(const BulletSystem *system);
size_t bullet_system_get_capacity(const BulletSystem *system);

#endif
