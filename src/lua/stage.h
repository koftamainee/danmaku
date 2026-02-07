#ifndef LUA_STAGE_H
#define LUA_STAGE_H

#include <lua.h>
#include <stdbool.h>

typedef struct {
  lua_State *co;
  int wait_frames;
  bool finished;
} LuaStage;

bool lua_stage_load(lua_State *L, const char *path, LuaStage *stage);
void lua_stage_update(LuaStage *stage);

#endif
