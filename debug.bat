ECHO OFF
cargo fmt
cargo clippy
cargo build
copy /y config.ron target\debug
cargo run