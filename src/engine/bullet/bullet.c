#include "bullet.h"
#include "bullet_id.h"
#include "bullet_system.h"
#include "log.h"
#include <assert.h>
#include <cglm/vec2.h>

bool bullet_set_speed(BulletSystem *sys, BulletID id, float speed) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->speed = speed;
  return true;
}

bool bullet_set_accel(BulletSystem *sys, BulletID id, float accel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->accel = accel;
  return true;
}

bool bullet_set_min_speed(BulletSystem *sys, BulletID id, float min_speed) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  if (b->max_speed < min_speed) {
    log_error("Tried to set max_speed < min_speed");
    return false;
  }

  b->min_speed = min_speed;
  return true;
}

bool bullet_set_max_speed(BulletSystem *sys, BulletID id, float max_speed) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  if (max_speed < b->min_speed) {
    log_error("Tried to set min_speed > max_speed");
    return false;
  }

  b->max_speed = max_speed;
  return true;
}

bool bullet_set_speed_limits(BulletSystem *sys, BulletID id, float min_speed,
                             float max_speed) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  if (max_speed < min_speed) {
    log_error("Tried to set max_speed < min_speed");
    return false;
  }

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->max_speed = max_speed;
  b->min_speed = min_speed;
  return true;
}

bool bullet_set_angular_vel(BulletSystem *sys, BulletID id, float angular_vel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angular_vel = angular_vel;
  return true;
}

bool bullet_set_angular_accel(BulletSystem *sys, BulletID id,
                              float angular_accel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angular_accel = angular_accel;
  return true;
}

bool bullet_set_min_angular_vel(BulletSystem *sys, BulletID id,
                                float min_angular_vel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  if (b->max_angular_vel < min_angular_vel) {
    log_error("Tried to set min_angular_vel > min_angular_vel");
    return false;
  }

  b->min_angular_vel = min_angular_vel;
  return true;
}

bool bullet_set_max_angular_vel(BulletSystem *sys, BulletID id,
                                float max_angular_vel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  if (max_angular_vel < b->min_angular_vel) {
    log_error("Tried to set max_angular_vel < min_angular_vel");
    return false;
  }

  b->max_angular_vel = max_angular_vel;
  return true;
}

bool bullet_set_angular_vel_limits(BulletSystem *sys, BulletID id,
                                   float min_angular_vel,
                                   float max_angular_vel) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  if (max_angular_vel < min_angular_vel) {
    log_error("Tried to set max_angular_vel < min_angular_vel");
    return false;
  }

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->min_angular_vel = min_angular_vel;
  b->max_angular_vel = max_angular_vel;
  return true;
}

bool bullet_set_angle(BulletSystem *sys, BulletID id, float angle) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angle = angle;
  return true;
}

bool bullet_add_angle(BulletSystem *sys, BulletID id, float angle) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angle += angle;
  return true;
}

bool bullet_aim(BulletSystem *sys, BulletID id) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angle = 0; // TODO: point at the player
  return true;
}

bool bullet_set_parent_offset(BulletSystem *sys, BulletID id, vec2 offset) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  glm_vec2_copy(offset, b->parent_offset);
  return true;
}

bool bullet_attach_to(BulletSystem *sys, BulletID id, BulletID parent,
                      vec2 offset) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->parent = parent;
  glm_vec2_copy(offset, b->parent_offset);
  return true;
}

bool bullet_set_lifetime(BulletSystem *sys, BulletID id, int lifetime) {
  assert(sys != NULL);
  assert(!bullet_id_is_null(id));

  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->lifetime = lifetime;
  return true;
}
