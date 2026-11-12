#!/usr/bin/env bash

# This is meant to be run as root

echo testvm | ssh rozanne@vsock%648372915 -- bash -c 'bootc switch --transport containers-storage'
