telnet:
    telnet 127.0.0.1 8080

buildrun:
    RUST_LOG=debug RUSTFLAGS="-A warnings" cargo build
    ./target/debug/ro-rust-v2

check:
    RUSTFLAGS="-A warnings" cargo check --workspace

run:
    RUST_LOG=debug ./target/debug/ro-rust-v2

diesel_migration:
    diesel migration run
