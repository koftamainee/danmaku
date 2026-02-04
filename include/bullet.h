#pragma once

#include <cglm/cglm.h>
#define MAX_BULLETS_COUNT (50000)

typedef struct {
  vec2 position;
  vec2 velocity;
  unsigned int lifetime;
} Bullet;

void bullet_set_angle(int id, float angle);

void bullet_set_speed(int id, float speed);

void bullet_set_velocity(int id, float vx, float vy);

void bullet_add_speed(int id, float ds);

void bullet_rotate(int id, float d_angle);

float bullet_get_angle(int id);

float bullet_get_speed(int id);

float bullet_get_vx(int id);

float bullet_get_vy(int id);

int bullet_get_lifetime(int id);

void bullet_set_lifetime(int id, int lifetime);
