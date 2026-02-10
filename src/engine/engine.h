#ifndef ENGINE_ENGINE_H
#define ENGINE_ENGINE_H

#include "config/config.h"
typedef struct Engine Engine;

Engine *engine_create(const Configuration *config);
void engine_run(Engine *engine);
void engine_destroy(Engine *engine);

#endif // !ENGINE_ENGINE_H
