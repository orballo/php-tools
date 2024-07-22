@default:
    just --list

@test:
    cargo test \
    --package parser \
    --no-fail-fast \
    --test literals

@review:
    cargo insta review