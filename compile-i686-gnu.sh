#!/bin/sh

host_root_src_dir="$(cd "$(dirname "$0")" && pwd)"

docker run -v "${host_root_src_dir}:/src" mcandre/docker-rustup:i686-gnu sh -c "cd /src && cargo build --release --target i686-unknown-linux-gnu"
