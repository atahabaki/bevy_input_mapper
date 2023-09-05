alias f := finishing_touches

finishing_touches:
    cargo fmt
    cargo check
    cargo clippy
    cargo test