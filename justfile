set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

run:
   just packages/puniyu/run
run-server:
    just crates/puniyu_server/run
build:
    just packages/puniyu/build
test:
    cargo test