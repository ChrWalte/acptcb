#!/bin/sh

# wait for the docker
dockerd > /dev/null 2>&1 &
while ! docker info >/dev/null 2>&1; do sleep 1; done

# start acptcb
exec /usr/local/bin/acptcb
