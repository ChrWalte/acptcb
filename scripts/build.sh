
#!/bin/bash
set -ex

docker build \
    -f ./containerfile \
    -t acptcb:latest \
    --build-arg build_type=debug \
    --compress .
