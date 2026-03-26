set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

run:
   just puniyu/run
run-server:
    just crates/puniyu_server/run
build:
    just puniyu/build
test:
    cargo test
publish:
    cargo workspaces publish --publish-interval 10 --no-git-push