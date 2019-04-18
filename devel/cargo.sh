#! /usr/bin/bash

SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd .. && pwd)"
PARAMS=$@

podman run --net=host --rm -it -v $SRC_DIR:/devel:z rems:dev \
	bash -c "cd /devel && cargo $PARAMS"
