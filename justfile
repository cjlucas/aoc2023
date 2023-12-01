alias r := run
alias t := test

default: test

run BIN:
    cargo run --bin {{BIN}}

test:
    cargo test

watch-test:
    bacon test
