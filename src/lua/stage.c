#include "lua/stage.h"
#include "log.h"
#include "lua/env.h"
#include <assert.h>
#include <lauxlib.h>
#include <lua.h>
#include <stdbool.h>
#include <time.h>

bool lua_stage_load(lua_State *L, const char *path, LuaStage *stage) {
  assert(L != NULL);
  assert(path != NULL);
  assert(stage != NULL);

  lua_State *co = lua_newthread(L);
  if (co == NULL) {
    log_error("Failed to create Lua coroutine");
    return false;
  }

  if (luaL_loadfile(co, path) != LUA_OK) {
    lua_pop(L, 1);
    log_error("Failed to load %s file into coroutine", path);
    return false;
  }

  if (lua_pcall(co, 0, 1, 0)) {
    lua_pop(L, 1);
    log_error("Failed to pcal %s file", path);
    return false;
  }

  if (!lua_istable(co, -1)) {
    lua_pop(co, 1);
    log_error("%s file not returning a table", path);
    return false;
  }

  // TODO extract all useful fields from table to some kind of a struct

  lua_getfield(co, -1, "run");

  if (!lua_isfunction(co, -1)) {
    lua_pop(co, 2);
    log_error("'run' field from table is not a function: %s", path);
    return false;
  }

  lua_remove(co, -2);

  lua_push_engine_table(co, true, 0);

  stage->co = co;
  stage->wait_frames = 0;
  stage->finished = false;

  log_info("%s lua stage loaded", path);

  return true;
}

void lua_stage_update(LuaStage *stage) {
  assert(stage != NULL);

  if (stage->wait_frames > 0) {
    stage->wait_frames--;
    return;
  }

  int nres = 0;
  int status = lua_resume(stage->co, NULL, 1, &nres);

  if (status == LUA_YIELD) {
    if (lua_gettop(stage->co) >= 1 && lua_isinteger(stage->co, -1)) {
      stage->wait_frames = (int)lua_tointeger(stage->co, -1);
      lua_pop(stage->co, 1);
    }
  } else {
    stage->finished = true;

    if (status != LUA_OK) {
      const char *msg = lua_tostring(stage->co, -1);
      if (msg) {
        log_error("Lua stage error: %s", msg);
      }
      lua_pop(stage->co, 1);
    }
  }
}
