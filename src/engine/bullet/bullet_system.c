#include "bullet_system.h"
#include "bullet.h"
#include "bullet_id.h"
#include <assert.h>
#include <float.h>
#include <log.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#define SCREEN_W 1000
#define SCREEN_H 1000

struct BulletSystem {
  Bullet *bullets;
  size_t *generations;
  size_t *free_stack;
  size_t capacity;
  size_t free_count;
  size_t *render_list;
  size_t render_count;
};

static inline void push_free(BulletSystem *sys, size_t index) {
  assert(sys != NULL);

  assert(sys->free_count < sys->capacity && "Free stack overflow");

  sys->free_stack[sys->free_count++] = index;
}

static inline size_t pop_free(BulletSystem *sys) {
  assert(sys != NULL);

  if (sys->free_count == 0) {
    return SIZE_MAX;
  }
  return sys->free_stack[--sys->free_count];
}

static inline bool bullet_outside_screen(const Bullet *b) {
  assert(b != NULL);

  return b->position[0] < -500.0f || b->position[1] < -500.0f ||
         b->position[0] > SCREEN_W + 500.0f ||
         b->position[1] > SCREEN_H + 500.0f;
}

static void kill_bullet_by_index(BulletSystem *sys, size_t index) {
  assert(sys != NULL);
  assert(index < sys->capacity && "Index out of bounds for internal funtion");

  Bullet *b = &sys->bullets[index];
  if (b->lifetime == 0) {
    return;
  }

  b->lifetime = 0;
  push_free(sys, index);
  sys->generations[index]++;
}

static void bullet_system_compact_render_list(BulletSystem *system) {
  assert(system != NULL);

  size_t write = 0;

  for (size_t read = 0; read < system->render_count; read++) {
    size_t idx = system->render_list[read];

    if (system->bullets[idx].lifetime != 0) {
      system->render_list[write++] = idx;
    }
  }

  system->render_count = write;
}

static void update_root_bullet_movement(Bullet *b) {
  assert(b != NULL);

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
}

BulletSystem *bullet_system_create(size_t capacity) {
  if (capacity == 0) {
    log_error("Cannot create bullet system with capacity 0");
    return NULL;
  }

  BulletSystem *sys = calloc(1, sizeof(BulletSystem));
  if (sys == NULL) {
    log_error("Failed to allocate bullet system");
    return NULL;
  }

  sys->bullets = calloc(capacity, sizeof(Bullet));
  sys->generations = calloc(capacity, sizeof(size_t));
  sys->free_stack = calloc(capacity, sizeof(size_t));
  sys->render_list = calloc(capacity, sizeof(size_t));

  if (sys->bullets == NULL || sys->generations == NULL ||
      sys->free_stack == NULL || sys->render_list == NULL) {
    log_error("Failed to allocate bullet system arrays");
    free(sys->bullets);
    free(sys->generations);
    free(sys->free_stack);
    free(sys->render_list);
    free(sys);
    return NULL;
  }

  sys->capacity = capacity;
  sys->free_count = capacity;
  sys->render_count = 0;

  for (size_t i = 0; i < capacity; i++) {
    sys->free_stack[i] = capacity - 1 - i;
    sys->generations[i] = 0;
    sys->bullets[i].lifetime = 0;
  }

  log_info("Bullet system created with capacity %zu", capacity);
  return sys;
}

void bullet_system_destroy(BulletSystem *system) {
  if (system == NULL) {
    return;
  }

  free(system->bullets);
  free(system->generations);
  free(system->free_stack);
  free(system->render_list);
  free(system);

  log_info("Bullet system destroyed");
}

BulletID bullet_system_spawn(BulletSystem *system, const Bullet *init) {
  assert(system != NULL);
  assert(init != NULL);

  size_t index = pop_free(system);
  if (index == SIZE_MAX) {
    log_warn("Cannot spawn bullet: system is full (%zu bullets)",
             system->capacity);
    return BULLET_ID_NULL;
  }

  Bullet *b = &system->bullets[index];

  *b = *init;

  Bullet *parent = bullet_system_get(system, b->parent);
  if (parent != NULL) {
    glm_vec2_copy(parent->position, b->position);
    glm_vec2_add(b->position, b->parent_offset, b->position);
  }

  system->generations[index]++;
  system->render_list[system->render_count++] = index;

  BulletID id = {.index = index, .generation = system->generations[index]};

  return id;
}

void bullet_system_kill(BulletSystem *system, BulletID id) {
  assert(system != NULL);

  assert(!bullet_id_is_null(id));

  assert(id.index < system->capacity);

  assert(system->generations[id.index] == id.generation);

  kill_bullet_by_index(system, id.index);
}

Bullet *bullet_system_get(BulletSystem *system, BulletID id) {
  assert(system != NULL);

  if (bullet_id_is_null(id)) {
    return NULL;
  }

  assert(id.index < system->capacity);

  if (system->generations[id.index] != id.generation) {
    log_warn("Tried to access stale bullet. Consider fixing it");
    return NULL;
  }

  Bullet *bullet = &system->bullets[id.index];
  return (bullet->lifetime != 0) ? bullet : NULL;
}

bool bullet_system_is_alive(BulletSystem *system, BulletID id) {
  assert(system != NULL);
  assert(!bullet_id_is_null(id));

  return bullet_system_get(system, id) != NULL;
}

void bullet_system_update(BulletSystem *system) {
  assert(system != NULL);

  for (size_t i = 0; i < system->capacity; i++) {
    Bullet *b = &system->bullets[i];

    if (b->lifetime == 0) {
      continue;
    }

    if (bullet_id_is_null(b->parent)) {
      update_root_bullet_movement(b);

      if (bullet_outside_screen(b)) {
        kill_bullet_by_index(system, i);
        continue;
      }
    }

    if (b->lifetime == 1) {
      kill_bullet_by_index(system, i);
    }

    if (b->lifetime > 0) {
      b->lifetime--;
    }
  }

  for (size_t i = 0; i < system->capacity; i++) {
    Bullet *b = &system->bullets[i];

    if (b->lifetime == 0 || bullet_id_is_null(b->parent)) {
      continue;
    }

    if (b->angular_vel != 0.0f) {
      float dx = b->parent_offset[0];
      float dy = b->parent_offset[1];

      float radius = sqrtf(dx * dx + dy * dy);
      float angle = atan2f(dy, dx) + b->angular_vel;

      b->parent_offset[0] = cosf(angle) * radius;
      b->parent_offset[1] = sinf(angle) * radius;
    }
  }

  bool updated = true;
  int iteration = 0;
  const int max_iterations = 10;

  while (updated && iteration < max_iterations) {
    updated = false;
    iteration++;

    for (size_t i = 0; i < system->capacity; i++) {
      Bullet *b = &system->bullets[i];

      if (b->lifetime == 0 || bullet_id_is_null(b->parent)) {
        continue;
      }

      Bullet *parent = bullet_system_get(system, b->parent);
      if (parent == NULL) {
        b->parent = BULLET_ID_NULL;
        continue;
      }

      float new_x = parent->position[0] + b->parent_offset[0];
      float new_y = parent->position[1] + b->parent_offset[1];

      if (b->position[0] != new_x || b->position[1] != new_y) {
        b->position[0] = new_x;
        b->position[1] = new_y;
        updated = true;
      }

      if (bullet_outside_screen(b)) {
        kill_bullet_by_index(system, i);
      }
    }
  }

  if (iteration >= max_iterations) {
    log_warn("Parent update iteration limit reached");
  }

  bullet_system_compact_render_list(system);
}

void bullet_system_clear(BulletSystem *system) {
  assert(system != NULL);

  for (size_t i = 0; i < system->capacity; i++) {
    if (system->bullets[i].lifetime != 0) {
      kill_bullet_by_index(system, i);
    }
  }

  system->render_count = 0;
}

void bullet_system_foreach_render_order(BulletSystem *system,
                                        BulletIteratorFn callback,
                                        void *user_data) {
  assert(system != NULL);
  assert(callback != NULL);
  // user_data can be NULL

  for (size_t i = 0; i < system->render_count; i++) {
    size_t idx = system->render_list[i];
    Bullet *b = &system->bullets[idx];
    if (b->lifetime != 0) {
      callback(b, user_data);
    }
  }
}

void bullet_system_foreach(BulletSystem *system, BulletIteratorFn callback,
                           void *user_data) {
  assert(system != NULL);
  assert(callback != NULL);
  // user_data can be NULL

  for (size_t i = 0; i < system->capacity; i++) {
    Bullet *b = &system->bullets[i];
    if (b->lifetime != 0) {
      callback(b, user_data);
    }
  }
}

size_t bullet_system_count_active(const BulletSystem *system) {
  assert(system != NULL);
  return system->capacity - system->free_count;
}

size_t bullet_system_count_free(const BulletSystem *system) {
  assert(system != NULL);
  return system->free_count;
}

size_t bullet_system_get_capacity(const BulletSystem *system) {
  assert(system != NULL);
  return system->capacity;
}
