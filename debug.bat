ECHO OFF
cargo build
copy /y config.ron target\debug
cargo run