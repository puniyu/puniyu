set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

run: 
    cargo run --bin puniyu
run-dev: 
    cargo run --bin puniyu --config .cargo/local.toml

test:
    just crates/puniyu_macros/
    cargo test
