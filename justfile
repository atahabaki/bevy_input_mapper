alias f := finishing_touches
alias g := gen_doc

gen_doc:
    cargo doc --all-features --open
finishing_touches:
    cargo fmt
    cargo check
    cargo clippy
    cargo test