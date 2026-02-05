log_inc = include_directories('include')


log_lib = static_library(
  'log',
  'src/log.c',
  include_directories: log_inc,
  c_args: ['-Wno-format-nonliteral']
)


log_dep = declare_dependency(
  link_with: log_lib,
  include_directories: log_inc
)
