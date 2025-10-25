set windows-shell := ["powershell.exe", "-c"]
run:
   cargo run --bin puniyu
run-server:
    cargo run --bin puniyu_server
build:
    cargo build --release --bin puniyu
release:
    cargo workspaces publish --publish-interval 10 --no-git-commit --no-git-push --no-git-tag --registry crates-io