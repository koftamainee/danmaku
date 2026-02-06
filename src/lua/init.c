#include "lua/init.h"
#include "log.h"
#include "lua/env.h"
#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>

lua_State *lua_system_init(void) {
  lua_State *L = luaL_newstate();
  if (L == NULL) {
    log_error("Failed to init Lua state");
    return NULL;
  }

  luaL_openlibs(L);

  lua_register_bullet(L);

  return L;
}

void lua_system_free(lua_State *L) {
  if (L != NULL) {
    lua_close(L);
  }
}
