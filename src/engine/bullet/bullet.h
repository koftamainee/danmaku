#ifndef ENGINE_BULLET_H
#define ENGINE_BULLET_H

#include "bullet_id.h"
#include <cglm/cglm.h>
#include <stdbool.h>

typedef struct BulletSystem BulletSystem;

typedef struct {
  vec2 position;
  vec2 velocity;

  float speed;
  float accel;
  float min_speed;
  float max_speed;

  float angle;
  float angular_vel;
  float angular_accel;
  float min_angular_vel;
  float max_angular_vel;

  BulletID parent;
  vec2 parent_offset;

  int lifetime;

  char sprite[64];
} Bullet;

bool bullet_set_speed(BulletSystem *sys, BulletID id, float speed);
bool bullet_set_accel(BulletSystem *sys, BulletID id, float accel);
bool bullet_set_min_speed(BulletSystem *sys, BulletID id, float min_speed);
bool bullet_set_max_speed(BulletSystem *sys, BulletID id, float max_speed);
bool bullet_set_speed_limits(BulletSystem *sys, BulletID id, float min_speed,
                             float max_speed);

bool bullet_set_angular_vel(BulletSystem *sys, BulletID id, float angular_vel);
bool bullet_set_angular_accel(BulletSystem *sys, BulletID id,
                              float angular_accel);
bool bullet_set_min_angular_vel(BulletSystem *sys, BulletID id,
                                float min_angular_vel);
bool bullet_set_max_angular_vel(BulletSystem *sys, BulletID id,
                                float max_angular_vel);
bool bullet_set_angular_vel_limits(BulletSystem *sys, BulletID id,
                                   float min_angular_vel,
                                   float max_angular_vel);

bool bullet_set_angle(BulletSystem *sys, BulletID id, float angle);
bool bullet_add_angle(BulletSystem *sys, BulletID id, float angle);
bool bullet_aim(BulletSystem *sys, BulletID id);

bool bullet_set_parent_offset(BulletSystem *sys, BulletID id, vec2 offset);
bool bullet_attach_to(BulletSystem *sys, BulletID id, BulletID parent,
                      vec2 offset);

bool bullet_set_lifetime(BulletSystem *sys, BulletID id, int lifetime);

#endif
