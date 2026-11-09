project(
  'rozanne',
  'rust',
  version : '0.1.0',
  meson_version : '>= 1.3.0',
  default_options : ['rust_std=2024', 'warning_level=3'],
  license: 'BSD-3-Clause'
)

prefixdir = get_option('prefix')
libexecdir = prefixdir / get_option('libexecdir')
bindir = prefixdir / get_option('bindir')

fs = import('fs')
if not fs.is_absolute(prefixdir)
  error('Prefix is not absolute: "@0@"'.format(prefixdir))
endif
datadir = prefixdir / get_option('datadir')

polkit_rules_dir = get_option('polkit_rules_dir')
if polkit_rules_dir == ''
  polkit_rules_dir = datadir / 'polkit-1' / 'rules.d'
endif

polkit_policy_dir = get_option('polkit_policy_dir')
if polkit_policy_dir == ''
  polkit_policy_dir = datadir / 'polkit-1' / 'actions'
endif

systemd = dependency('systemd', version: '>= 242')

systemdsystemunitdir = get_option('systemdsystemunitdir')
if systemdsystemunitdir == ''
  if systemd.found()
    systemdsystemunitdir = systemd.get_variable(
      pkgconfig: 'systemdsystemunitdir',
      pkgconfig_define: ['prefix', prefixdir],
    )
  else
    # deliberately lib and not based on get_option('libdir'):
    # this should not be lib64 or lib/x86_64-linux-gnu
    systemdsystemunitdir = prefixdir / 'lib/systemd/system'
  endif
endif


systemduserunitdir = get_option('systemduserunitdir')
if systemduserunitdir == ''
  if systemd.found()
    systemduserunitdir = systemd.get_variable(
      pkgconfig: 'systemduserunitdir',
      pkgconfig_define: ['prefix', prefixdir],
    )
  else
    # deliberately lib and not based on get_option('libdir'):
    # this should not be lib64 or lib/x86_64-linux-gnu
    systemduserunitdir = prefixdir / 'lib/systemd/user'
  endif
endif

subdir('data')

if get_option('cargo_build').enabled()
  subdir('src')
endif

summary({
  'datadir': datadir,
  'bindir': bindir,
  'systemdsystemunitdir': systemdsystemunitdir,
  'systemduserunitdir': systemduserunitdir,
})
