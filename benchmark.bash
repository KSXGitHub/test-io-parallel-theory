#!/bin/bash
set -o errexit -o pipefail -o nounset
cd "$(dirname "$0")"

pretty-exec -- cargo build --release

PATH="$(pwd)/target/release:$PATH"
export PATH

commands=(
  'test-io-parallel-theory serial'
  'test-io-parallel-theory parallel'
  'test-io-parallel-theory mixed'
)

pretty-exec -- hyperfine --prepare 'rm -rf output' --warmup 3 "${commands[@]}"
