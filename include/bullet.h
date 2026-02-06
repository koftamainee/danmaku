#pragma once

#include "bullet_id.h"
#include <cglm/cglm.h>

// typedef enum { Absolute, Relative, Player } BulletAngleType;

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

bool bullet_set_speed(BulletID id, float speed);
bool bullet_set_accel(BulletID id, float accel);
bool bullet_set_min_speed(BulletID id, float min_speed);
bool bullet_set_max_speed(BulletID id, float max_speed);
bool bullet_set_speed_limits(BulletID id, float min_speed, float max_speed);

bool bullet_set_angular_vel(BulletID id, float angular_vel);
bool bullet_set_angular_accel(BulletID id, float angular_accel);
bool bullet_set_min_angular_vel(BulletID id, float min_angular_vel);
bool bullet_set_max_angular_vel(BulletID id, float max_angular_vel);
bool bullet_set_angular_vel_limits(BulletID id, float min_angular_vel,
                                   float max_angular_vel);

bool bullet_set_angle(BulletID id, float angle);
bool bullet_aim(BulletID id);

bool bullet_set_parent_offset(BulletID id, vec2 offset);
bool bullet_attach_to(BulletID id, BulletID parent, vec2 offset);

bool bullet_set_lifetime(BulletID id, int lifetime);
