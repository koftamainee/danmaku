#include "lua/wrappers.h"
#include "engine/bullet/bullet.h"
#include "engine/bullet/bullet_id.h"
#include "engine/bullet/bullet_system.h"
#include <assert.h>
#include <lauxlib.h>
#include <lua.h>
#include <string.h>

typedef struct {
  BulletID id;
} LuaBullet;

static const char *BULLET_SYSTEM_KEY = "bullet_system_ptr";

BulletSystem *lua_get_bullet_system(lua_State *L) {
  assert(L != NULL);

  lua_pushlightuserdata(L, (void *)BULLET_SYSTEM_KEY);
  lua_gettable(L, LUA_REGISTRYINDEX);
  BulletSystem *sys = lua_touserdata(L, -1);
  lua_pop(L, 1);
  return sys;
}

static inline LuaBullet *check_lua_bullet(lua_State *L, int expected_args,
                                          const char *func_name) {
  assert(L != NULL);
  assert(func_name != NULL);

  int nargs = lua_gettop(L);
  if (nargs != expected_args) {
    luaL_error(L, "%s: expected %d argument(s), got %d", func_name,
               expected_args - 1, nargs - 1);
  }

  LuaBullet *b = (LuaBullet *)luaL_checkudata(L, 1, "Bullet");
  if (b == NULL) {
    luaL_error(L, "%s: invalid bullet userdata", func_name);
  }
  return b;
}

int l_bullet_set_speed(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_speed()");
  float speed = (float)luaL_checknumber(L, 2);

  if (!bullet_set_speed(sys, b->id, speed)) {
    return luaL_error(L, "set_speed(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_accel(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_accel()");
  float accel = (float)luaL_checknumber(L, 2);

  if (!bullet_set_accel(sys, b->id, accel)) {
    return luaL_error(L, "set_accel(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_min_speed(lua_State *L) {
  BulletSystem *sys = lua_get_bullet_system(L);
  if (!sys) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_min_speed()");
  float min_speed = (float)luaL_checknumber(L, 2);

  if (!bullet_set_min_speed(sys, b->id, min_speed)) {
    return luaL_error(L, "set_min_speed(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_max_speed(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_max_speed()");
  float max_speed = (float)luaL_checknumber(L, 2);

  if (!bullet_set_max_speed(sys, b->id, max_speed)) {
    return luaL_error(L, "set_max_speed(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_speed_limits(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 3, "set_speed_limits()");
  float min_speed = (float)luaL_checknumber(L, 2);
  float max_speed = (float)luaL_checknumber(L, 3);

  if (!bullet_set_speed_limits(sys, b->id, min_speed, max_speed)) {
    return luaL_error(L, "set_speed_limits(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_angular_vel(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_angular_vel()");
  float angular_vel = (float)luaL_checknumber(L, 2);

  if (!bullet_set_angular_vel(sys, b->id, angular_vel)) {
    return luaL_error(L, "set_angular_vel(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_angular_accel(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_angular_accel()");
  float angular_accel = (float)luaL_checknumber(L, 2);

  if (!bullet_set_angular_accel(sys, b->id, angular_accel)) {
    return luaL_error(L, "set_angular_accel(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_min_angular_vel(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_min_angular_vel()");
  float min_angular_vel = (float)luaL_checknumber(L, 2);

  if (!bullet_set_min_angular_vel(sys, b->id, min_angular_vel)) {
    return luaL_error(L, "set_min_angular_vel(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_max_angular_vel(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_max_angular_vel()");
  float max_angular_vel = (float)luaL_checknumber(L, 2);

  if (!bullet_set_max_angular_vel(sys, b->id, max_angular_vel)) {
    return luaL_error(L, "set_max_angular_vel(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_angular_vel_limits(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 3, "set_angular_vel_limits()");
  float min_angular_vel = (float)luaL_checknumber(L, 2);
  float max_angular_vel = (float)luaL_checknumber(L, 3);

  if (!bullet_set_angular_vel_limits(sys, b->id, min_angular_vel,
                                     max_angular_vel)) {
    return luaL_error(L,
                      "set_angular_vel_limits(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_angle(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_angle()");
  float angle = (float)luaL_checknumber(L, 2);

  if (!bullet_set_angle(sys, b->id, angle)) {
    return luaL_error(L, "set_angle(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_aim(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 1, "aim()");

  if (!bullet_aim(sys, b->id)) {
    return luaL_error(L, "aim(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_parent_offset(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_parent_offset()");

  luaL_checktype(L, 2, LUA_TTABLE);

  vec2 offset;

  lua_getfield(L, 2, "x");
  if (!lua_isnumber(L, -1)) {
    lua_pop(L, 1);
    lua_rawgeti(L, 2, 1);
  }
  offset[0] = (float)luaL_checknumber(L, -1);
  lua_pop(L, 1);

  lua_getfield(L, 2, "y");
  if (!lua_isnumber(L, -1)) {
    lua_pop(L, 1);
    lua_rawgeti(L, 2, 2);
  }
  offset[1] = (float)luaL_checknumber(L, -1);
  lua_pop(L, 1);

  if (!bullet_set_parent_offset(sys, b->id, offset)) {
    return luaL_error(L, "set_parent_offset(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_attach_to(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = (LuaBullet *)luaL_checkudata(L, 1, "Bullet");
  if (b == NULL) {
    return luaL_error(L, "attach_to(): 'parent' must be a bullet handle");
  }

  BulletID parent_id = BULLET_ID_NULL;
  if (!lua_isnil(L, 2)) {
    LuaBullet *parent = (LuaBullet *)luaL_testudata(L, 2, "Bullet");
    if (!parent) {
      return luaL_error(L, "attach_to(): 'parent' must be a bullet handle");
    }
    parent_id = parent->id;
  }

  vec2 offset = {0.0f, 0.0f};
  if (lua_gettop(L) >= 4) {
    offset[0] = (float)luaL_checknumber(L, 3);
    offset[1] = (float)luaL_checknumber(L, 4);
  }

  if (!bullet_attach_to(sys, b->id, parent_id, offset)) {
    return luaL_error(L, "attach_to(): failed, bullet or parent may be dead");
  }

  return 0;
}

int l_bullet_detach(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 1, "detach()");
  vec2 offset = {0.0f, 0.0f};

  if (!bullet_attach_to(sys, b->id, BULLET_ID_NULL, offset)) {
    return luaL_error(L, "detach(): failed, bullet may be dead");
  }

  return 0;
}

int l_bullet_set_lifetime(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  LuaBullet *b = check_lua_bullet(L, 2, "set_lifetime()");
  int lifetime = (int)luaL_checkinteger(L, 2);

  if (!bullet_set_lifetime(sys, b->id, lifetime)) {
    return luaL_error(L, "set_lifetime(): failed, bullet may be dead");
  }

  return 0;
}

int l_engine_time(lua_State *L) {
  assert(L != NULL);

  lua_pushinteger(L, 0); // TODO: fixme somehow pls pls pplsls
  return 1;
}

int l_engine_wait(lua_State *L) {
  assert(L != NULL);

  int frames = 0;

  if (lua_gettop(L) >= 1) {
    if (!lua_isinteger(L, 1)) {
      return luaL_error(L, "wait() expects an integer argument");
    }
    frames = (int)lua_tointeger(L, 1);
    if (frames < 0) {
      return luaL_error(L, "wait() frames must be non-negative");
    }
  }

  lua_pushinteger(L, frames);
  return lua_yield(L, 1);
}

int l_engine_yield(lua_State *L) {
  assert(L != NULL);

  lua_pushinteger(L, 0);
  return lua_yield(L, 1);
}

int l_spawn_bullet(lua_State *L) {
  assert(L != NULL);

  BulletSystem *sys = lua_get_bullet_system(L);
  if (sys == NULL) {
    return luaL_error(L, "Bullet system not initialized");
  }

  if (lua_gettop(L) != 1) {
    return luaL_error(L, "spawn_bullet() expects exactly 1 argument, got %d",
                      lua_gettop(L));
  }

  if (!lua_istable(L, 1)) {
    return luaL_error(L, "spawn_bullet() expects a table argument");
  }

  Bullet init;
  memset(&init, 0, sizeof(Bullet));

  init.parent = BULLET_ID_NULL;
  init.lifetime = -1;

  lua_getfield(L, 1, "sprite");
  if (!lua_isstring(L, -1)) {
    return luaL_error(
        L, "spawn_bullet(): 'sprite' field is required and must be a string");
  }
  const char *sprite = lua_tostring(L, -1);
  strncpy(init.sprite, sprite, sizeof(init.sprite) - 1);
  init.sprite[sizeof(init.sprite) - 1] = '\0';
  lua_pop(L, 1);

  lua_getfield(L, 1, "parent");
  bool has_parent = !lua_isnil(L, -1);
  if (has_parent) {
    LuaBullet *parent_id = (LuaBullet *)lua_touserdata(L, -1);
    if (parent_id == NULL) {
      lua_pop(L, 1);
      return luaL_error(L, "spawn_bullet(): 'parent' must be a bullet handle");
    }
    init.parent = parent_id->id;
  }
  lua_pop(L, 1);

  lua_getfield(L, 1, "x");
  lua_getfield(L, 1, "y");
  if (!has_parent) {
    if (!lua_isnumber(L, -2) || !lua_isnumber(L, -1)) {
      return luaL_error(
          L, "spawn_bullet(): 'x' and 'y' are required when no parent is set");
    }
  }
  init.position[0] = lua_isnumber(L, -2) ? (float)lua_tonumber(L, -2) : 0.0f;
  init.position[1] = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 2);

  lua_getfield(L, 1, "speed");
  init.speed = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "accel");
  init.accel = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "min_speed");
  init.min_speed = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "max_speed");
  init.max_speed = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "angle");
  init.angle = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "angle_type");
  if (lua_isinteger(L, -1)) {
    int angle_type = (int)lua_tointeger(L, -1);
    (void)angle_type;
  }
  lua_pop(L, 1);

  lua_getfield(L, 1, "angular_vel");
  init.angular_vel = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "angular_accel");
  init.angular_accel = lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "min_angular_vel");
  init.min_angular_vel =
      lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "max_angular_vel");
  init.max_angular_vel =
      lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
  lua_pop(L, 1);

  lua_getfield(L, 1, "parent_offset");
  if (lua_istable(L, -1)) {
    lua_getfield(L, -1, "x");
    init.parent_offset[0] =
        lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
    lua_pop(L, 1);

    lua_getfield(L, -1, "y");
    init.parent_offset[1] =
        lua_isnumber(L, -1) ? (float)lua_tonumber(L, -1) : 0.0f;
    lua_pop(L, 1);
  }
  lua_pop(L, 1);

  lua_getfield(L, 1, "lifetime");
  if (lua_isinteger(L, -1)) {
    init.lifetime = (int)lua_tointeger(L, -1);
  }
  lua_pop(L, 1);

  BulletID id = bullet_system_spawn(sys, &init);

  if (bullet_id_is_null(id)) {
    lua_pushnil(L);
    return 1;
  }

  LuaBullet *ud = lua_newuserdata(L, sizeof(LuaBullet));
  ud->id = id;
  luaL_setmetatable(L, "Bullet");

  return 1;
}
