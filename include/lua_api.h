#pragma once

#include <lua.h>
#include <stdbool.h>

typedef struct {
  int lua_ref;
  int wait_frames;
  bool is_done;
} GameplaySection;

lua_State *lua_init(void);

int load_section(lua_State *L, const char *section_name,
                 GameplaySection *section);

int update_section(lua_State *L, GameplaySection *section);
