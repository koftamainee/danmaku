#include "bullet_system.h"
#include "bullet.h"
#include "bullet_id.h"
#include <cglm/vec2.h>
#include <math.h>
#include <string.h>

#define SCREEN_W 1000
#define SCREEN_H 1000

Bullet bullets[MAX_BULLETS_COUNT];
static size_t generations[MAX_BULLETS_COUNT];
static size_t free_stack[MAX_BULLETS_COUNT];
size_t free_count;

size_t to_render[MAX_BULLETS_COUNT];
size_t to_render_count;

static inline void push_free(size_t index) { free_stack[free_count++] = index; }

static inline size_t pop_free(void) {
  if (free_count == 0) {
    return SIZE_MAX;
  }
  return free_stack[--free_count];
}

bool bullet_is_alive(BulletID id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  return true;
}

Bullet *bullet_get(BulletID id) {

  if (bullet_id_is_null(id) || id.index >= MAX_BULLETS_COUNT ||
      generations[id.index] != id.generation)
    return NULL;
  return &bullets[id.index];
}

void bullet_system_init(void) {
  free_count = MAX_BULLETS_COUNT;
  to_render_count = 0;

  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    bullets[i].lifetime = 0;
    free_stack[i] = MAX_BULLETS_COUNT - 1 - (size_t)i;
    generations[i] = 0;
  }
}

BulletID spawn_bullet(const Bullet *init) {
  size_t index = pop_free();
  if (index == SIZE_MAX) {
    return BULLET_ID_NULL;
  }

  Bullet *b = &bullets[index];

  if (init != NULL) {
    *b = *init;
  } else {
    memset(b, 0, sizeof(Bullet));
  }

  Bullet *parent = bullet_get(b->parent);
  if (parent != NULL) {
    glm_vec2_copy(parent->position, b->position);
    glm_vec2_add(b->position, b->parent_offset, b->position);
  }

  generations[index]++;

  to_render[to_render_count++] = index;

  return (BulletID){.index = index, .generation = generations[index]};
}

static inline void kill_bullet_by_id(size_t index) {
  bullets[index].lifetime = 0;

  push_free(index);
  generations[index]++;
}

void kill_bullet(BulletID id) {
  if (bullet_id_is_null(id)) {
    return;
  }

  if (id.index >= MAX_BULLETS_COUNT) {
    return;
  }

  if (generations[id.index] != id.generation) {
    return;
  }

  kill_bullet_by_id(id.index);
}

static inline bool bullet_outside_screen(const Bullet *b) {
  return b->position[0] < -500.0f || b->position[1] < -500.0f ||
         b->position[0] > SCREEN_W || b->position[1] > SCREEN_H;
}

void bullet_system_update(void) {
  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    Bullet *b = bullets + i;
    if (b->lifetime == 0) {
      continue;
    }
    if (bullet_id_is_null(b->parent)) {
      b->speed += b->accel;
      if (b->min_speed != 0) {
        b->speed = fmaxf(b->speed, b->min_speed);
      }
      if (b->max_speed != 0) {
        b->speed = fminf(b->speed, b->max_speed);
      }
      b->angular_vel += b->angular_accel;
      if (b->min_angular_vel != 0) {
        b->angular_vel = fmaxf(b->angular_vel, b->min_angular_vel);
      }
      if (b->max_angular_vel != 0) {
        b->angular_vel = fminf(b->angular_vel, b->max_angular_vel);
      }
      b->angle += b->angular_vel;
      b->position[0] += cosf(b->angle) * b->speed;
      b->position[1] += sinf(b->angle) * b->speed;
      if (bullet_outside_screen(b)) {
        kill_bullet_by_id((size_t)i);
        continue;
      }
    }
    if (b->lifetime > 0) {
      b->lifetime--;
    }
    if (b->lifetime == 0) {
      kill_bullet_by_id((size_t)i);
    }
  }

  bool updated = true;
  int max_iterations = 10;
  int iteration = 0;

  while (updated && iteration < max_iterations) {
    updated = false;
    iteration++;

    for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
      Bullet *b = bullets + i;
      if (b->lifetime == 0 || bullet_id_is_null(b->parent)) {
        continue;
      }

      Bullet *p = bullet_get(b->parent);
      if (p != NULL) {
        float dx = b->parent_offset[0];
        float dy = b->parent_offset[1];
        float radius = sqrtf(dx * dx + dy * dy);
        float angle = atan2f(dy, dx);
        angle += b->angular_vel;
        b->parent_offset[0] = cosf(angle) * radius;
        b->parent_offset[1] = sinf(angle) * radius;

        float new_x = p->position[0] + b->parent_offset[0];
        float new_y = p->position[1] + b->parent_offset[1];

        if (b->position[0] != new_x || b->position[1] != new_y) {
          b->position[0] = new_x;
          b->position[1] = new_y;
          updated = true;
        }
      } else {
        b->parent = BULLET_ID_NULL;
      }

      if (bullet_outside_screen(b)) {
        kill_bullet_by_id((size_t)i);
        continue;
      }
      if (b->lifetime > 0) {
        b->lifetime--;
      }
      if (b->lifetime == 0) {
        kill_bullet_by_id((size_t)i);
      }
    }
  }
}
void bullet_system_compact_render_list(void) {
  size_t write = 0;

  for (size_t read = 0; read < to_render_count; read++) {
    size_t idx = to_render[read];

    if (bullets[idx].lifetime != 0) {
      to_render[write++] = idx;
    }
  }

  to_render_count = write;
}
