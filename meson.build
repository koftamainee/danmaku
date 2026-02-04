project('danmaku', 'c',
        version : '0.0.1',
        default_options : ['warning_level=3', 'c_std=c11'])

cc = meson.get_compiler('c')

sdl3_dep = dependency('sdl3')
cglm_dep = dependency('cglm')
lua_dep = dependency('lua')
inih_dep = dependency('inih')

m_dep = cc.find_library('m', required: true)

incdir = include_directories('include')

srcs = ['src/main.c', 'src/config.c', 'src/log.c', 'src/sdl.c', 'src/fps.c', 'src/game.c', 'src/lua_api.c', 'src/bullet.c']
danmaku = executable('danmaku', srcs, dependencies: [sdl3_dep, cglm_dep, m_dep, lua_dep, inih_dep], include_directories: incdir, install : true)


run_target('run',
  command: danmaku,
)

