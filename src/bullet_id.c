#include "bullet_id.h"

bool bullet_id_equal(BulletID first, BulletID second) {
  return first.index == second.index && first.generation == second.generation;
}

bool bullet_id_is_null(BulletID id) {
  return bullet_id_equal(id, BULLET_ID_NULL);
}
