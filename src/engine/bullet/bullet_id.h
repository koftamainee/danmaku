#ifndef ENGINE_BULLET_ID_H
#define ENGINE_BULLET_ID_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct {
  size_t index;
  size_t generation;
} BulletID;

#define BULLET_ID_NULL ((BulletID){.index = SIZE_MAX, .generation = 0})

bool bullet_id_equal(BulletID first, BulletID second);
bool bullet_id_is_null(BulletID id);

#endif
