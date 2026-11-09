cargo_bin = find_program('cargo')

cargo_target_dir = meson.project_build_root() / 'src'

if get_option('buildtype') == 'release'
  cargo_extra_opts = ['--release']
  rust_target = 'release'
else
  cargo_extra_opts = []
  rust_target = 'debug'
endif

cargo_artifact_dir = cargo_target_dir / rust_target

bin_name = meson.project_name()

cargo_build_bins = custom_target(
  'cargo-build-binaries',
  build_by_default: true,
  build_always_stale: true,
  console: true,
  env: {
    'CARGO_HOME': meson.project_build_root() / 'cargo-home',
    'CARGO_TARGET_DIR': cargo_target_dir,
  },
  command: [
    cargo_bin,
    'build',
    '--manifest-path', meson.project_source_root() / 'Cargo.toml',
    '--bin', bin_name,
    cargo_extra_opts,
    '&&',
    'cp',
    cargo_artifact_dir / bin_name,
    '@OUTDIR@',
  ],
  output: [
    bin_name,
  ],
  install: true,
  install_dir: [
    bindir,
  ],
)
