#include "spritesheet.h"
#include "log.h"
#include <SDL3_image/SDL_image.h>
#include <assert.h>
#include <cjson/cJSON.h>
#include <libgen.h>
#include <stdlib.h>
#include <string.h>
#include <uthash.h>

#define PATH_MAX 4096

struct SpriteSheet {
  SDL_Texture *texture;
  SpriteEntry *sprites;
};

static char *read_file(const char *path) {
  assert(path != NULL);

  FILE *f = fopen(path, "rb");
  if (f == NULL) {
    log_error("Failed to open %s", path);
    return NULL;
  }

  fseek(f, 0, SEEK_END);
  long len = ftell(f);
  if (len == 0) {
    log_error("File %s is empty", path);
  }
  fseek(f, 0, SEEK_SET);

  char *buf = malloc((size_t)len + 1);
  if (buf == NULL) {
    log_error("Failed to allocate memory for %s content", path);
    fclose(f);
    return NULL;
  }

  fread(buf, 1, (size_t)len, f);
  buf[len] = '\0';
  fclose(f);
  return buf;
}

SpriteSheet *spritesheet_load(SDL_Renderer *renderer, const char *json_path) {
  assert(json_path != NULL);
  assert(renderer != NULL);

  SpriteSheet *spritesheet = calloc(1, sizeof(SpriteSheet));
  if (spritesheet == NULL) {
    log_error("Failed to allocate memory for spritesheet");
    return NULL;
  }

  char *text = read_file(json_path);
  if (text == NULL) {
    log_error("Failed to read file: %s", json_path);
    free(spritesheet);
    return NULL;
  }

  // TODO: validate JSON via schema

  cJSON *root = cJSON_Parse(text);
  free(text);
  if (root == NULL) {
    log_error("Failed to parse JSON: %s", json_path);
    free(spritesheet);
    return NULL;
  }

  const cJSON *texture_item = cJSON_GetObjectItem(root, "texture");
  if (!cJSON_IsString(texture_item)) {
    log_fatal("JSON missing 'texture' field");
    cJSON_Delete(root);
    free(spritesheet);
    return NULL;
  }

  char texture_path[PATH_MAX];
  char json_dir[PATH_MAX];
  strncpy(json_dir, json_path, PATH_MAX - 1);
  json_dir[PATH_MAX - 1] = '\0';
  strncpy(texture_path, dirname(json_dir), PATH_MAX - 1);
  texture_path[PATH_MAX - 1] = '\0';

  strncat(texture_path, "/", PATH_MAX - strlen(texture_path) - 1);
  strncat(texture_path, texture_item->valuestring,
          PATH_MAX - strlen(texture_path) - 1);

  spritesheet->texture = IMG_LoadTexture(renderer, texture_path);
  if (spritesheet->texture == NULL) {
    log_fatal("Failed to load texture: %s", texture_path);
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
      log_warn("Skipping invalid sprite");
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

    entry->region.src.x = (float)x_item->valueint;
    entry->region.src.y = (float)y_item->valueint;
    entry->region.src.w = (float)w_item->valueint;
    entry->region.src.h = (float)h_item->valueint;

    HASH_ADD_KEYPTR(hh, spritesheet->sprites, entry->name, strlen(entry->name),
                    entry);
  }

  cJSON_Delete(root);

  log_info("Spritesheet from %s loaded", json_path);

  return spritesheet;
}

const SpriteRegion *spritesheet_get(const SpriteSheet *spritesheet,
                                    const char *name) {
  assert(spritesheet != NULL);
  assert(name != NULL);

  SpriteEntry *entry = NULL;
  if (spritesheet == NULL) {
    return NULL;
  }

  HASH_FIND_STR(spritesheet->sprites, name, entry);

  return entry ? &entry->region : NULL;
}

SDL_Texture *spritesheet_texture(const SpriteSheet *sheet) {
  assert(sheet != NULL);

  return sheet->texture;
}

void spritesheet_destroy(SpriteSheet *sheet) {
  if (sheet == NULL) {
    return;
  }
  SpriteEntry *e = NULL, *tmp = NULL;

  HASH_ITER(hh, sheet->sprites, e, tmp) {
    HASH_DEL(sheet->sprites, e);
    free(e->name);
    free(e);
  }

  SDL_DestroyTexture(sheet->texture);
  free(sheet);
}
