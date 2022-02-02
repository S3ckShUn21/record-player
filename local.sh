#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

cargo build --release

cp ./target/release/player testbench/player
cp ./target/release/webserver testbench/webserver