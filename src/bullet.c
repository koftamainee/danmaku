#include "bullet.h"

extern Bullet bullets[MAX_BULLETS_COUNT];

static inline Bullet *bullet_get(int id) {
  if (id < 0 || id >= MAX_BULLETS_COUNT)
    return NULL;
  return &bullets[id];
}

void bullet_set_angle(int id, float angle) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  float speed = glm_vec2_norm(b->velocity);
  glm_vec2_copy((vec2){cosf(angle) * speed, sinf(angle) * speed}, b->velocity);
}

void bullet_set_speed(int id, float speed) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  float angle = atan2f(b->velocity[1], b->velocity[0]);
  glm_vec2_copy((vec2){cosf(angle) * speed, sinf(angle) * speed}, b->velocity);
}

void bullet_set_velocity(int id, float vx, float vy) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  b->velocity[0] = vx;
  b->velocity[1] = vy;
}

void bullet_add_speed(int id, float ds) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  float speed = glm_vec2_norm(b->velocity);
  if (speed == 0.0f) {
    return;
  }

  speed += ds;
  if (speed < 0.0f) {
    speed = 0.0f;
  }

  float angle = atan2f(b->velocity[1], b->velocity[0]);
  glm_vec2_copy((vec2){cosf(angle) * speed, sinf(angle) * speed}, b->velocity);
}

void bullet_rotate(int id, float d_angle) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  float speed = glm_vec2_norm(b->velocity);
  if (speed == 0.0f) {
    return;
  }

  float angle = atan2f(b->velocity[1], b->velocity[0]);
  angle += d_angle;

  glm_vec2_copy((vec2){cosf(angle) * speed, sinf(angle) * speed}, b->velocity);
}

float bullet_get_angle(int id) {
  Bullet *b = bullet_get(id);
  if (b == NULL)
    return 0.0f;
  return atan2f(b->velocity[1], b->velocity[0]);
}

float bullet_get_speed(int id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return 0.0f;
  }
  return glm_vec2_norm(b->velocity);
}

float bullet_get_vx(int id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return 0.0f;
  }
  return b->velocity[0];
}

float bullet_get_vy(int id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return 0.0f;
  }
  return b->velocity[1];
}
int bullet_get_lifetime(int id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return 0;
  }

  return b->lifetime;
}

void bullet_set_lifetime(int id, int lifetime) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return;
  }

  b->lifetime = lifetime;
}
