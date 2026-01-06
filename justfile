set windows-shell := ["powershell.exe", "-c"]
set shell := ["bash", "-cu"]

run:
   just packages/puniyu/run
run-server:
    just packages/puniyu_server/run
build:
    just packages/puniyu/build
test:
    cargo test
publish:
    cargo workspaces publish --publish-interval 10 --no-git-push