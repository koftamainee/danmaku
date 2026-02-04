#include "lua_api.h"
#include "game.h"

#include "bullet.h"
#include "log.h"
#include <cglm/types.h>
#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>

#include <cglm/cglm.h>

extern Bullet bullets[MAX_BULLETS_COUNT];

static int lua_spawn_bullet(lua_State *L) {
  double x = luaL_checknumber(L, 1);
  double y = luaL_checknumber(L, 2);
  double angle = luaL_checknumber(L, 3); // in radians
  double speed = luaL_checknumber(L, 4);
  int lifetime = luaL_checkinteger(L, 5);

  vec2 position = {x, y};
  vec2 velocity = {speed * sin(angle), speed * cos(angle)};

  int result = spawn_bullet(bullets, position, velocity, lifetime);
  lua_pushinteger(L, result);
  return 1;
}

static int lua_wait(lua_State *L) {
  int frames = luaL_checkinteger(L, 1);
  if (frames < 0) {
    frames = 0;
  }

  return lua_yield(L, 1);
}

static int lua_yield_frame(lua_State *L) {
  int nargs = lua_gettop(L);
  if (nargs != 0) {
    return luaL_error(L, "yield() takes no arguments (got %d)", nargs);
  }

  lua_pushinteger(L, 0);
  return lua_yield(L, 1);
}

static int lua_bullet_set_angle(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float angle = luaL_checknumber(L, 2);

  bullet_set_angle(id, angle);
  return 0;
}

static int lua_bullet_set_speed(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float speed = luaL_checknumber(L, 2);

  bullet_set_speed(id, speed);
  return 0;
}

static int lua_bullet_set_velocity(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float vx = luaL_checknumber(L, 2);
  float vy = luaL_checknumber(L, 3);

  bullet_set_velocity(id, vx, vy);
  return 0;
}

static int lua_bullet_add_speed(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float ds = luaL_checknumber(L, 2);

  bullet_add_speed(id, ds);
  return 0;
}

static int lua_bullet_rotate(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float d_angle = luaL_checknumber(L, 2);

  bullet_rotate(id, d_angle);
  return 0;
}

static int lua_bullet_get_angle(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float angle = bullet_get_angle(id);
  lua_pushnumber(L, angle);
  return 1;
}

static int lua_bullet_get_speed(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float speed = bullet_get_speed(id);
  lua_pushnumber(L, speed);
  return 1;
}

static int lua_bullet_get_velocity(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  float vx = bullet_get_vx(id);
  float vy = bullet_get_vy(id);
  lua_pushnumber(L, vx);
  lua_pushnumber(L, vy);
  return 2;
}

static int lua_bullet_set_lifetime(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  int lifetime = luaL_checkinteger(L, 2);
  bullet_set_lifetime(id, lifetime);
  return 0;
}

static int lua_bullet_get_lifetime(lua_State *L) {
  int id = luaL_checkinteger(L, 1);
  int lifetime = bullet_get_lifetime(id);
  lua_pushinteger(L, lifetime);
  return 1;
}

lua_State *lua_init(void) {

  lua_State *L = luaL_newstate();
  if (L == NULL) {
    log_fatal("Failed to init Lua");
    return NULL;
  }
  luaL_openlibs(L);

  lua_register(L, "spawn_bullet", lua_spawn_bullet);
  lua_register(L, "wait", lua_wait);
  lua_register(L, "yield", lua_yield_frame);
  lua_register(L, "bullet_set_angle", lua_bullet_set_angle);
  lua_register(L, "bullet_set_speed", lua_bullet_set_speed);
  lua_register(L, "bullet_set_velocity", lua_bullet_set_velocity);
  lua_register(L, "bullet_add_speed", lua_bullet_add_speed);
  lua_register(L, "bullet_rotate", lua_bullet_rotate);
  lua_register(L, "bullet_get_angle", lua_bullet_get_angle);
  lua_register(L, "bullet_get_speed", lua_bullet_get_speed);
  lua_register(L, "bullet_get_velocity", lua_bullet_get_velocity);
  lua_register(L, "bullet_set_lifetime", lua_bullet_set_lifetime);
  lua_register(L, "bullet_get_lifetime", lua_bullet_get_lifetime);
  return L;
}

int load_section(lua_State *L, const char *section_name,
                 GameplaySection *section) {
  if (!section_name || !section)
    return 1;

  lua_getglobal(L, section_name);
  if (!lua_isfunction(L, -1)) {
    log_error("%s function not found in Lua", section_name);
    lua_pop(L, 1);
    return 1;
  }

  lua_State *co = lua_newthread(L);
  lua_pushvalue(L, -2);
  lua_xmove(L, co, 1);

  section->lua_ref = luaL_ref(L, LUA_REGISTRYINDEX);
  lua_pop(L, 1);
  section->wait_frames = 0;

  return 0;
}

int update_section(lua_State *L, GameplaySection *section) {
  if (!L || !section)
    return 1;

  if (section->wait_frames > 0) {
    section->wait_frames--;
    return 0;
  }

  lua_rawgeti(L, LUA_REGISTRYINDEX, section->lua_ref);
  lua_State *co = lua_tothread(L, -1);

  if (co == NULL) {
    log_error("Failed to get coroutine from registry");
    lua_pop(L, 1);
    return 1;
  }

  int nres = 0;
  int status =
      lua_resume(co, L, 0, &nres); // Also fixed: pass L as the parent state

  if (status == LUA_YIELD) {
    // check if coroutine yielded a wait time
    if (lua_gettop(co) >= 1 && lua_isinteger(co, -1)) {
      section->wait_frames = lua_tointeger(co, -1);
      lua_pop(co, 1);
    }
  } else if (status == LUA_OK) {
    // coroutine finished
    log_info("Section completed");
    luaL_unref(L, LUA_REGISTRYINDEX, section->lua_ref);
    section->lua_ref = LUA_REFNIL;
    lua_pop(L, 1); // Pop the thread reference
    return 0;
  } else {
    // error occurred
    const char *err = lua_tostring(co, -1);
    log_error("Lua error: %s", err ? err : "unknown error");
    lua_pop(co, 1);
    luaL_unref(L, LUA_REGISTRYINDEX, section->lua_ref);
    section->lua_ref = LUA_REFNIL;
    lua_pop(L, 1); // Pop the thread reference
    return 1;
  }

  lua_pop(L, 1); // Pop the thread reference after successful yield
  return 0;
}
