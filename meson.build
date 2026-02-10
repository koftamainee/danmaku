project('danmaku', 'c',
  version : '0.0.1',
  default_options : [
    'warning_level=3',
    'c_std=c11',         
    'b_ndebug=if-release',
  ]
)

add_project_arguments([
  '-Wpedantic',
  '-Wshadow',
  '-Wconversion',
  '-Wsign-conversion',
  '-Wstrict-prototypes',
  '-Wmissing-prototypes',
  '-Wundef',
  '-Wformat=2'
], language: 'c')

log_sub = subproject('log')             
log_dep = log_sub.get_variable('log_dep')  


# TODO: static doesnt work for inih and lua
# probably need to build them from scratch too
cjson_dep = dependency('libcjson', static: true)
inih_dep  = dependency('inih', static: true)
cglm_dep  = dependency('cglm', static: true)
lua_dep   = dependency('lua', static: true)


sdl3_dep     = dependency('sdl3')
sdl3_img_dep = dependency('SDL3_image')
uthash_dep   = dependency('uthash')

cc = meson.get_compiler('c')
m_dep = cc.find_library('m', required: true)

danmaku_deps = [
  log_dep,
  sdl3_dep,
  sdl3_img_dep,
  cglm_dep,
  lua_dep,
  inih_dep,
  cjson_dep,
  uthash_dep,
  m_dep
]

srcdir = 'src'
incdir = include_directories(srcdir)

srcs = files(
  'src/main.c',
  'src/platform/sdl.c',
  'src/config/config.c',
  'src/engine/framerate/limiter.c',
  'src/engine/bullet/bullet.c',
  'src/engine/bullet/bullet_id.c',
  'src/engine/bullet/bullet_system.c',
  'src/engine/render/spritesheet.c',
  'src/engine/render/bullet_renderer.c',
  'src/lua/env.c',
  'src/lua/lua_system.c',
  'src/lua/stage.c',
  'src/lua/wrappers.c'
)

danmaku = executable(
  'danmaku',
  srcs,
  dependencies: danmaku_deps,
  include_directories: [incdir],
  install: true
)

