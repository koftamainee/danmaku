#include "spritesheet.h"
#include "log.h"
#include "subprojects/cJSON-1.7.19/cJSON.h"
#include "uthash.h"
#include <SDL3_image/SDL_image.h>
#include <stdlib.h>
#include <string.h>

struct SpriteSheet {
  SDL_Texture *texture;
  SpriteEntry *sprites;
};

static char *read_file(const char *path) {
  FILE *f = fopen(path, "rb");
  if (!f)
    return NULL;

  fseek(f, 0, SEEK_END);
  long len = ftell(f);
  fseek(f, 0, SEEK_SET);

  char *buf = malloc(len + 1);
  if (!buf) {
    fclose(f);
    return NULL;
  }

  fread(buf, 1, len, f);
  buf[len] = '\0';
  fclose(f);
  return buf;
}

SpriteSheet *spritesheet_load(SDL_Renderer *renderer, const char *json_path) {
  SpriteSheet *spritesheet = calloc(1, sizeof(SpriteSheet));
  if (spritesheet == NULL) {
    // TODO: maybe die
    log_fatal("Out of memory");
    return NULL;
  }

  char *text = read_file(json_path);
  if (text == NULL) {
    log_error("Failed to read file: %s", json_path);
    free(spritesheet);
    return NULL;
  }

  cJSON *root = cJSON_Parse(text);
  free(text);
  if (root == NULL) {
    log_error("Failed to parse JSON: %s", json_path);
    return NULL;
  }

  const cJSON *texture_item = cJSON_GetObjectItem(root, "texture");
  if (!cJSON_IsString(texture_item)) {
    log_fatal("JSON missing 'texture' field");
    cJSON_Delete(root);
    free(spritesheet);
    return NULL;
  }

  spritesheet->texture = IMG_LoadTexture(renderer, texture_item->valuestring);
  if (spritesheet->texture == NULL) {
    log_fatal("Failed to load texture: %s", texture_item->valuestring);
    cJSON_Delete(root);
    free(spritesheet);
    return NULL;
  }

  cJSON *sprites = cJSON_GetObjectItem(root, "sprites");
  if (!cJSON_IsArray(sprites)) {
    log_fatal("'sprites' field missing or not an array");
    SDL_DestroyTexture(spritesheet->texture);
    cJSON_Delete(root);
    free(spritesheet);
    return NULL;
  }

  cJSON *sprite = NULL;
  cJSON_ArrayForEach(sprite, sprites) {
    cJSON *name_item = cJSON_GetObjectItem(sprite, "name");
    cJSON *x_item = cJSON_GetObjectItem(sprite, "x");
    cJSON *y_item = cJSON_GetObjectItem(sprite, "y");
    cJSON *w_item = cJSON_GetObjectItem(sprite, "width");
    cJSON *h_item = cJSON_GetObjectItem(sprite, "height");

    if (!cJSON_IsString(name_item) || !cJSON_IsNumber(x_item) ||
        !cJSON_IsNumber(y_item) || !cJSON_IsNumber(w_item) ||
        !cJSON_IsNumber(h_item)) {
      log_warn("Skipping invalid frame");
      continue;
    }

    SpriteEntry *entry = malloc(sizeof(SpriteEntry));
    if (!entry) {
      log_fatal("Out of memory allocating sprite");
      return NULL;
    }

    size_t len = strlen(name_item->valuestring) + 1;
    entry->name = malloc(len);
    if (!entry->name) {
      log_fatal("Out of memory allocating sprite name");
      return NULL;
    }
    strcpy(entry->name, name_item->valuestring); // copy string

    entry->region.src.x = x_item->valueint;
    entry->region.src.y = y_item->valueint;
    entry->region.src.w = w_item->valueint;
    entry->region.src.h = h_item->valueint;

    HASH_ADD_KEYPTR(hh, spritesheet->sprites, entry->name, strlen(entry->name),
                    entry);
  }

  cJSON_Delete(root);

  return spritesheet;
}

const SpriteRegion *spritesheet_get(const SpriteSheet *spritesheet,
                                    const char *name) {
  SpriteEntry *entry = NULL;
  if (spritesheet == NULL) {
    return NULL;
  }

  HASH_FIND_STR(spritesheet->sprites, name, entry);

  return entry ? &entry->region : NULL;
}

SDL_Texture *spritesheet_texture(const SpriteSheet *sheet) {
  return sheet->texture;
}

void spritesheet_free(SpriteSheet *sheet) {
  SpriteEntry *e = NULL, *tmp = NULL;

  HASH_ITER(hh, sheet->sprites, e, tmp) {
    HASH_DEL(sheet->sprites, e);
    free(e->name);
    free(e);
  }

  SDL_DestroyTexture(sheet->texture);
  free(sheet);
}
