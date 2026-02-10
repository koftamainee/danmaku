#include "lua/lua_system.h"
#include "lua/env.h"
#include "lua_system.h"
#include <assert.h>
#include <lauxlib.h>
#include <log.h>
#include <lualib.h>
#include <stdint.h>

void *LUA_ENGINE_KEY;

lua_State *lua_system_create(Engine *engine) {
  assert(engine != NULL);

  lua_State *L = luaL_newstate();
  if (L == NULL) {
    log_error("Failed to init Lua state");
    return NULL;
  }

  luaL_openlibs(L);

  lua_pushlightuserdata(L, LUA_ENGINE_KEY);
  lua_pushlightuserdata(L, engine);
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
