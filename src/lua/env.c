#include "lua/env.h"
#include "lua/wrappers.h"
#include <assert.h>
#include <lauxlib.h>
#include <lua.h>

void lua_push_engine_table(lua_State *L, bool is_sandbox, int rng_seed) {
  assert(L != NULL);

  lua_newtable(L);

  lua_pushcfunction(L, l_engine_wait);
  lua_setfield(L, -2, "wait");

  lua_pushcfunction(L, l_engine_time);
  lua_setfield(L, -2, "time");

  lua_pushcfunction(L, l_engine_yield);
  lua_setfield(L, -2, "yield");

  lua_pushcfunction(L, l_spawn_bullet);
  lua_setfield(L, -2, "spawn_bullet");

  lua_newtable(L);
  lua_pushinteger(L, ENGINE_ANGLE_ABSOLUTE);
  lua_setfield(L, -2, "absolute");

  lua_pushinteger(L, ENGINE_ANGLE_RELATIVE);
  lua_setfield(L, -2, "relative");

  lua_pushinteger(L, ENGINE_ANGLE_PLAYER);
  lua_setfield(L, -2, "player");

  lua_setfield(L, -2, "angle");

  if (!is_sandbox) {
    lua_pushinteger(L, rng_seed);
    lua_setfield(L, -2, "rng_seed");
  }
}

void lua_register_bullet(lua_State *L) {
  assert(L != NULL);

  luaL_newmetatable(L, "Bullet");

  static const luaL_Reg bullet_methods[] = {
      {"set_speed", l_bullet_set_speed},
      {"set_accel", l_bullet_set_accel},
      {"set_max_speed", l_bullet_set_max_speed},
      {"set_min_speed", l_bullet_set_min_speed},
      {"set_speed_limits", l_bullet_set_speed_limits},

      {"set_angle", l_bullet_set_angle},
      {"aim", l_bullet_aim},

      {"set_angular_vel", l_bullet_set_angular_vel},
      {"set_angular_accel", l_bullet_set_angular_accel},
      {"set_max_angular_vel", l_bullet_set_max_angular_vel},
      {"set_min_angular_vel", l_bullet_set_min_angular_vel},
      {"set_angular_vel_limits", l_bullet_set_angular_vel_limits},

      {"set_parent_offset", l_bullet_set_parent_offset},
      {"attach_to", l_bullet_attach_to},
      {"detach", l_bullet_detach},

      {"set_lifetime", l_bullet_set_lifetime},
      {NULL, NULL}};

  luaL_setfuncs(L, bullet_methods, 0);

  lua_pushvalue(L, -1);
  lua_setfield(L, -2, "__index");

  lua_pop(L, 1);
}
