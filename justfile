telnet:
    telnet 127.0.0.1 8080
run:
    RUST_LOG=debug RUSTFLAGS="-A warnings" cargo build
    ./target/debug/ro-rust-v2