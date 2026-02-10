#ifndef LUA_INIT_H
#define LUA_INIT_H

#include "engine/engine.h"
#include <lua.h>

typedef struct BulletSystem BulletSystem;

extern void *LUA_ENGINE_KEY;

lua_State *lua_system_create(Engine *engine);
void lua_system_destroy(lua_State *L);

#endif
