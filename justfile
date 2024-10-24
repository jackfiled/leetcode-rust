#!/usr/bin/env just --justfile

update:
    git pull

build: update
    cargo build --release

fmt: 
  cargo fmt

test: fmt
    cargo test

commit: test
    #!/usr/bin/env bash
    set -euxo pipefail
    time=$(date "+%Y%m%d")
    message="$time finished."
    
    git add -A
    git commit -m "$message"
    git push

pull id: build
    ./target/release/leetcode-rust {{id}}
