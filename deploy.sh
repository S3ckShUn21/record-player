#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=janus@janus
readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf

readonly TARGET_PLAYER_PATH=/home/janus/rust/record-player/player
readonly TARGET_WEBSERVER_PATH=/home/janus/rust/record-player/webserver

readonly LOCAL_PLAYER_PATH=./target/${TARGET_ARCH}/release/player
readonly LOCAL_WEBSERVER_PATH=./target/${TARGET_ARCH}/release/webserver

# Build the executables
cargo build --release --target=${TARGET_ARCH}

# Send the executables to the rpi
rsync ${LOCAL_PLAYER_PATH} ${TARGET_HOST}:${TARGET_PLAYER_PATH}
rsync ${LOCAL_WEBSERVER_PATH} ${TARGET_HOST}:${TARGET_WEBSERVER_PATH}

# Run the webserver
# ssh -t ${TARGET_HOST} ${TARGET_WEBSERVER_PATH}