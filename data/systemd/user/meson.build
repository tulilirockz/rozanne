if systemduserunitdir != ''
  configure_file(
    input: 'rozanne.service.in',
    output: 'rozanne.service',
    configuration: libexecdir_conf,
    install_dir: systemduserunitdir,
  )
  configure_file(
    input: 'rozanne.timer.in',
    output: 'rozanne.timer',
    configuration: libexecdir_conf,
    install_dir: systemduserunitdir,
  )
endif
