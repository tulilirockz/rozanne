#!/usr/bin/env bash

set -xeuo pipefail

# This is meant to be run as root

IMAGE_NAME="test-image:latest"
CONTAINERFILE_PATH="./Containerfile"

podman build -t "${IMAGE_NAME}" . -f "${CONTAINERFILE_PATH}"

if [ ! -e "${BUILD_BASE_DIR:-.}/bootable.img" ] ; then
    fallocate -l 20G "${BUILD_BASE_DIR:-.}/bootable.img"
fi

podman run \
    --rm --privileged --pid=host \
    -it \
    -v /sys/fs/selinux:/sys/fs/selinux \
    -v /etc/containers:/etc/containers:Z,ro \
    -v /var/lib/containers:/var/lib/containers:Z,rw \
    -v /dev:/dev \
    -v "${BUILD_BASE_DIR:-.}:/data" \
    --security-opt label=type:unconfined_t \
    "${IMAGE_NAME}" bootc install to-disk --generic-image --bootloader grub --via-loopback /data/bootable.img --filesystem btrfs --wipe --target-transport containers-storage --target-imgref "${IMAGE_NAME}"

# Now we rebuild the image without tmux so we can update :3

sed '/DELETEME-WITH-SED/d' "${CONTAINERFILE_PATH}" | podman build -t "${IMAGE_NAME}" . -f /dev/stdin
