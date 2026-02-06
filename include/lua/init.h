#pragma once

#include <lua.h>

lua_State *lua_system_init(void);

void lua_system_free(lua_State *L);
