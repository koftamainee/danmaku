#ifndef LUA_ENV_H
#define LUA_ENV_H

#include <lua.h>
#include <stdbool.h>

void lua_push_engine_table(lua_State *L, bool is_sandbox, int rng_seed);
void lua_register_bullet(lua_State *L);

#endif
