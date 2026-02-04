#include "game.h"
#include <string.h>

static inline int find_free_bullet_slot(const Bullet *bullets) {
  if (bullets == NULL)
    return -1;

  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    if (bullets[i].lifetime == 0)
      return i;
  }
  return -1;
}

int spawn_bullet(Bullet *bullets, vec2 position, vec2 velocity, int lifetime,
                 const char *sprite) {

  int slot = find_free_bullet_slot(bullets);
  if (slot < 0)
    return -1;

  Bullet *b = bullets + slot;
  glm_vec2_copy(position, b->position);
  glm_vec2_copy(velocity, b->velocity);
  b->lifetime = lifetime;

  if (sprite != NULL) {
    strncpy(b->sprite, sprite, sizeof(b->sprite) - 1);
    b->sprite[sizeof(b->sprite) - 1] = '\0';
  } else {
    b->sprite[0] = '\0';
  }

  return slot;
}

int update_bullets(Bullet *bullets, int *active_bullets_count) {
  if (bullets == NULL || active_bullets_count == NULL)
    return 1;

  for (int i = 0; i < MAX_BULLETS_COUNT; i++) {
    Bullet *b = bullets + i;
    if (b->lifetime == 0)
      continue;

    glm_vec2_add(b->position, b->velocity, b->position);

    if (b->lifetime == 1) {
      (*active_bullets_count)--;
    }

    b->lifetime--;
  }
  return 0;
}
