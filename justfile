set windows-shell := ["powershell.exe", "-c"]
run:
    cargo run --bin puniyu
run-server:
    cargo run --bin puniyu_server
build:
    cargo build --release --bin puniyu