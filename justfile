set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

run: 
    cargo run --bin puniyu

test:
    just crates/puniyu_macros/
    cargo test
