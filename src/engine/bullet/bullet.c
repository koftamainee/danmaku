#include "bullet.h"
#include "bullet_id.h"
#include "bullet_system.h"
#include <cglm/vec2.h>

bool bullet_set_speed(BulletSystem *sys, BulletID id, float speed) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->speed = speed;
  return true;
}

bool bullet_set_accel(BulletSystem *sys, BulletID id, float accel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->accel = accel;
  return true;
}

bool bullet_set_min_speed(BulletSystem *sys, BulletID id, float min_speed) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->min_speed = min_speed;
  return true;
}

bool bullet_set_max_speed(BulletSystem *sys, BulletID id, float max_speed) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->max_speed = max_speed;
  return true;
}

bool bullet_set_speed_limits(BulletSystem *sys, BulletID id, float min_speed,
                             float max_speed) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->max_speed = max_speed;
  b->min_speed = min_speed;
  return true;
}

bool bullet_set_angular_vel(BulletSystem *sys, BulletID id, float angular_vel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angular_vel = angular_vel;
  return true;
}

bool bullet_set_angular_accel(BulletSystem *sys, BulletID id,
                              float angular_accel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angular_accel = angular_accel;
  return true;
}

bool bullet_set_min_angular_vel(BulletSystem *sys, BulletID id,
                                float min_angular_vel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->min_angular_vel = min_angular_vel;
  return true;
}

bool bullet_set_max_angular_vel(BulletSystem *sys, BulletID id,
                                float max_angular_vel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->max_angular_vel = max_angular_vel;
  return true;
}

bool bullet_set_angular_vel_limits(BulletSystem *sys, BulletID id,
                                   float min_angular_vel,
                                   float max_angular_vel) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->min_angular_vel = min_angular_vel;
  b->max_angular_vel = max_angular_vel;
  return true;
}

bool bullet_set_angle(BulletSystem *sys, BulletID id, float angle) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angle = angle;
  return true;
}

bool bullet_aim(BulletSystem *sys, BulletID id) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->angle = 0; // TODO: point at the player
  return true;
}

bool bullet_set_parent_offset(BulletSystem *sys, BulletID id, vec2 offset) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  glm_vec2_copy(offset, b->parent_offset);
  return true;
}

bool bullet_attach_to(BulletSystem *sys, BulletID id, BulletID parent,
                      vec2 offset) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->parent = parent;
  glm_vec2_copy(offset, b->parent_offset);
  return true;
}

bool bullet_set_lifetime(BulletSystem *sys, BulletID id, int lifetime) {
  Bullet *b = bullet_system_get(sys, id);
  if (b == NULL) {
    return false;
  }

  b->lifetime = lifetime;
  return true;
}
