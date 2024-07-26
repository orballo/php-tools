@default:
    just --list

@run:
    cargo run --package parser

@test:
    cargo test \
    --package parser \
    --no-fail-fast \
    --test literals

@review:
    cargo insta review
