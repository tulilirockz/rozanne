#!/usr/bin/env bash

set -x

CONTAINERDIR="/var/lib/containers"
nohup vmbuddy --display console bootable.img --no-sandbox --no-tpm --vsock-cid 676767 -v "${CONTAINERDIR}:containers" -v ./cache:rozanne &
trap 'kill -9 $!' EXIT
sleep 20 # FIXME: please improve this somehow

sshpass -p testvm ssh root@vsock%676767 -- bash -c 'mkdir -p /var/lib/containers && mount -t 9p containers /var/lib/containers && mount -t 9p rozanne /mnt'
sshpass -p testvm ssh root@vsock%676767 -- /mnt/rozanne
ssh -t liveuser@vsock%676767 -- "sudo sh -c 'cp -f /mnt/rozanne.raw /run/extensions/rozanne.raw ; systemd-sysext refresh --mutable=ephemeral'"
