#include "lua/lua_system.h"
#include "engine/bullet/bullet_system.h"
#include "lua/env.h"
#include <assert.h>
#include <lauxlib.h>
#include <log.h>
#include <lualib.h>

static const char *BULLET_SYSTEM_KEY = "bullet_system_ptr";

lua_State *lua_system_create(BulletSystem *bullet_system) {
  assert(bullet_system != NULL);

  lua_State *L = luaL_newstate();
  if (L == NULL) {
    log_error("Failed to init Lua state");
    return NULL;
  }

  luaL_openlibs(L);

  lua_pushlightuserdata(L, (void *)BULLET_SYSTEM_KEY);
  lua_pushlightuserdata(L, bullet_system);
  lua_settable(L, LUA_REGISTRYINDEX);

  lua_register_bullet(L);

  log_info("Lua system created");
  return L;
}

void lua_system_destroy(lua_State *L) {
  if (L != NULL) {
    lua_close(L);
    log_info("Lua system destroyed");
  }
}
