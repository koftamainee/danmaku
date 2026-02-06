#include "bullet.h"
#include "bullet_id.h"
#include "bullet_system.h"
#include <cglm/vec2.h>

extern Bullet bullets[MAX_BULLETS_COUNT];

bool bullet_set_speed(BulletID id, float speed) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->speed = speed;

  return true;
}

bool bullet_set_accel(BulletID id, float accel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->accel = accel;

  return true;
}

bool bullet_set_min_speed(BulletID id, float min_speed) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->min_speed = min_speed;

  return true;
}

bool bullet_set_max_speed(BulletID id, float max_speed) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->max_speed = max_speed;

  return true;
}

bool bullet_set_speed_limits(BulletID id, float min_speed, float max_speed) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->max_speed = max_speed;
  b->min_speed = min_speed;

  return true;
}

bool bullet_set_angular_vel(BulletID id, float angular_vel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->angular_vel = angular_vel;

  return true;
}

bool bullet_set_angular_accel(BulletID id, float angular_accel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->angular_accel = angular_accel;

  return true;
}

bool bullet_set_min_angular_vel(BulletID id, float min_angular_vel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->min_angular_vel = min_angular_vel;

  return true;
}

bool bullet_set_max_angular_vel(BulletID id, float max_angular_vel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->max_angular_vel = max_angular_vel;

  return true;
}

bool bullet_set_angular_vel_limits(BulletID id, float min_angular_vel,
                                   float max_angular_vel) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->min_angular_vel = min_angular_vel;
  b->max_angular_vel = max_angular_vel;

  return true;
}

bool bullet_set_angle(BulletID id, float angle) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->angle = angle;

  return true;
}

bool bullet_aim(BulletID id) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->angle = 0; // TODO: fix when player is implemented

  return true;
}

bool bullet_set_parent_offset(BulletID id, vec2 offset) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  glm_vec2_copy(offset, b->parent_offset);

  return true;
}

bool bullet_attach_to(BulletID id, BulletID parent, vec2 offset) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->parent = parent;
  glm_vec2_copy(offset, b->parent_offset);

  return true;
}

bool bullet_set_lifetime(BulletID id, int lifetime) {
  Bullet *b = bullet_get(id);
  if (b == NULL) {
    return false;
  }

  b->lifetime = lifetime;

  return true;
}
