
// src/main.rs

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 解析参数并创建 Config 结构体
    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 运行 `run` 函数，并处理错误
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/*
cargo run -- Rust /Users/tengzhang/Documents/Core/rust-test/README.md
IGNORE_CASE=1 cargo run -- rUSt /Users/tengzhang/Documents/Core/rust-test/README.md

cargo test

*/