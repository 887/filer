project('a887.filer','rust',
  version: '0.1.0',
  meson_version: '>= 0.40.0',
  license: 'MPLv2'
)
i18n = import('i18n')

subdir('data')
subdir('src')
subdir('po')

cargo = find_program('cargo', required: true)
gresource = find_program('glib-compile-resources', required: true)
gschemas = find_program('glib-compile-schemas', required: true)

