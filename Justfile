@default:
    just --list

@run:
    cargo run

@watch :
    cargo watch -x 'run'

@test:
    cargo test

@review:
    cargo insta review
