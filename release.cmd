ECHO OFF
cargo build --release
copy /y config.ron target\release