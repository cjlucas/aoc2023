alias r := run
alias t := test

help:
    @just -l

run BIN:
    cargo run --release --bin {{ BIN }}

test:
    cargo test

watch-test:
    @bacon test
