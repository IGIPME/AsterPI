set windows-shell := ["powershell.exe"]
export RUST_BACKTRACE := "1"

# 展示可用的命令
@just:
    just --list

# 启动 AsterPI，非热重载
run:
    cargo leptos serve

# 启动 AsterPI，热重载
reload:
    cargo leptos watch

# 运行 Rust 测试
test:
    cargo test

# 运行 Cargo Check
check:
    cargo check -p api-cli
    cargo check -p app
    cargo check -p frontend
    cargo check -p kernel
    cargo check -p protocol
    cargo check -p server

# 调用 rustfmt 工具
fmt:
    cargo fmt --all -- --check

# 调用 clippy 工具
clippy:
    cargo clippy -p api-cli -- -D warnings
    cargo clippy -p app -- -D warnings
    cargo clippy -p frontend -- -D warnings
    cargo clippy -p kernel -- -D warnings
    cargo clippy -p protocol -- -D warnings
    cargo clippy -p server -- -D warnings

# 清理构建产物
[windows]
clean:
    cargo clean