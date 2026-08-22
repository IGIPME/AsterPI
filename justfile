set windows-shell := ["powershell.exe"]
export RUST_BACKTRACE := "1"

# 展示可用的命令
@just:
    just --list

# 运行 Rust 测试
test:
    cargo test

# 调用 rustfmt 工具
fmt:
    cargo fmt --all -- --check

# 调用 clippy 工具
clippy:
    cargo clippy -p a -- -D warnings

# 清理构建产物
[windows]
clean:
    cargo clean