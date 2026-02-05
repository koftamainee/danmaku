project('danmaku', 'c',
        version : '0.0.1',
        default_options : ['warning_level=3', 'c_std=c11'])



add_project_arguments(
  [
    '-Wall',
    '-Wextra',
    '-Wpedantic',
    '-Wshadow',
    '-Wconversion',
    '-Wsign-conversion',
    '-Wstrict-prototypes',
    '-Wmissing-prototypes',
    '-Wundef',
    '-Wformat=2'
  ],
  language: 'c'
)

subdir('third_party/log')



cc = meson.get_compiler('c')

sdl3_dep = dependency('sdl3')
sdl3_img_dep = dependency('SDL3_image')
cglm_dep = dependency('cglm')
lua_dep = dependency('lua')
inih_dep = dependency('inih')
cjson_dep = dependency('libcjson') 
uthash_dep = dependency('uthash')

m_dep = cc.find_library('m', required: true)

incdir = include_directories('include')

danmaku_deps = [
  sdl3_dep,
  sdl3_img_dep,
  cglm_dep,
  m_dep,
  lua_dep,
  inih_dep,
  cjson_dep,
  uthash_dep,
  log_dep
]


srcs = [
  'src/main.c',
  'src/config.c',
  'src/sdl.c',
  'src/fps.c',
  'src/game.c',
  'src/lua_api.c',
  'src/bullet.c',
  'src/spritesheet.c',
]

danmaku = executable(
  'danmaku',
  srcs,
  dependencies: danmaku_deps,
  include_directories: incdir,
  install: true
)


run_target('run',
  command: danmaku,
)

