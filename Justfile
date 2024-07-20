@default:
    just --list

@run parser:
    cargo run --package parser

@test parser:
    cargo test \
    --package parser \
    --no-fail-fast \
    --test lexical_grammar