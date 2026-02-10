#ifndef LUA_STAGE_H
#define LUA_STAGE_H

#include <lua.h>
#include <stdbool.h>

typedef struct LuaStage LuaStage;

LuaStage *lua_stage_create(lua_State *L, const char *path);
void lua_stage_destroy(LuaStage *stage);

void lua_stage_update(LuaStage *stage);

bool lua_stage_is_finished(LuaStage *stage);

#endif
