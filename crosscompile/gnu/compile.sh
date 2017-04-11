#!/bin/sh

host_root_src_dir="$(cd "$(dirname "$0")"/../.. && pwd)"

docker run -v "${host_root_src_dir}:/src" mcandre/ios7crypt-rs-build-bot:gnu sh -c "cd /src && cargo build --release --target x86_64-unknown-linux-gnu"
