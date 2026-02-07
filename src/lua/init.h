#ifndef LUA_INIT_H
#define LUA_INIT_H

#include <lua.h>

typedef struct BulletSystem BulletSystem;

lua_State *lua_system_init(BulletSystem *bullet_system);
void lua_system_destroy(lua_State *L);

#endif
