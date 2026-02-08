#ifndef LUA_WRAPPERS_H
#define LUA_WRAPPERS_H

#include <lua.h>

typedef struct BulletSystem BulletSystem;

typedef enum {
  ENGINE_ANGLE_ABSOLUTE = 0,
  ENGINE_ANGLE_RELATIVE = 1,
  ENGINE_ANGLE_PLAYER = 2,
} EngineAngle;

BulletSystem *lua_get_bullet_system(lua_State *L);

int l_bullet_set_speed(lua_State *L);
int l_bullet_set_accel(lua_State *L);
int l_bullet_set_min_speed(lua_State *L);
int l_bullet_set_max_speed(lua_State *L);
int l_bullet_set_speed_limits(lua_State *L);

int l_bullet_set_angular_vel(lua_State *L);
int l_bullet_set_angular_accel(lua_State *L);
int l_bullet_set_min_angular_vel(lua_State *L);
int l_bullet_set_max_angular_vel(lua_State *L);
int l_bullet_set_angular_vel_limits(lua_State *L);

int l_bullet_set_angle(lua_State *L);
int l_bullet_aim(lua_State *L);

int l_bullet_set_parent_offset(lua_State *L);
int l_bullet_attach_to(lua_State *L);
int l_bullet_detach(lua_State *L);

int l_bullet_set_lifetime(lua_State *L);

int l_spawn_bullet(lua_State *L);
int l_engine_time(lua_State *L);
int l_engine_wait(lua_State *L);
int l_engine_yield(lua_State *L);

#endif
