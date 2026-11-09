build_dir := env("BUILD_DIR", "_build")
install_prefix := env("INSTALL_PREFIX", "/usr")
just := just_executable()

alias setup := setup-builddir

setup-builddir:
    #!/usr/bin/env bash

    meson setup --wipe {{build_dir}} --prefix {{install_prefix}}

iterate:
    meson setup --wipe {{build_dir}} --prefix {{install_prefix}}
    meson compile -C {{build_dir}} --prefix {{install_prefix}}

sysext:
    set -x
    DESTDIR=rozanne-sysext meson install -C {{build_dir}}
    sysextbuddy --name rozanne {{build_dir}}/rozanne-sysext/ -o {{build_dir}}/rozanne.raw

host:
    dev -- just sysext
    sudo mkdir -p /run/extensions/
    sudo cp {{build_dir}}/rozanne.raw /run/extensions/rozanne.raw
    sudo systemd-sysext refresh
    
vm: sysext
    ssh -t liveuser@vsock%676767 -- "sudo sh -c 'cp -f /mnt/rozanne.raw /run/extensions/rozanne.raw ; systemd-sysext refresh --mutable=ephemeral'"
