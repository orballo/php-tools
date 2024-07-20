@default:
    just --list

@test:
    cargo test \
    --package parser \
    --no-fail-fast \
    --test lexical_grammar

@review:
    cargo insta review