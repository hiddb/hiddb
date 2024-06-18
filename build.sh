#/bin/bash

docker build -t hiddb_build .
docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/app -t hiddb_build

# cargo build --target=x86_64-unknown-linux-gnu --release
#RUSTFLAGS="-C linker=clang -C target-feature=+crt-static" cargo test --target=x86_64-unknown-linux-musl
# RUSTFLAGS='-C link-arg=-s -C target-feature=+crt-static' cargo test --target=x86_64-unknown-linux-musl
# RUSTFLAGS="-C target-feature=+crt-static" cargo test --target=x86_64-unknown-linux-musl
# RUSTFLAGS="-C linker=musl-gcc -C target-feature=-crt-static" cargo test --target=x86_64-unknown-linux-musl
