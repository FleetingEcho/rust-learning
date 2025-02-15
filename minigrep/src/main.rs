// use std::env;
// use std::process;
// use minigrep::Config;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });
//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }
// }

/*
cargo run -- Rust /Users/tengzhang/Documents/Core/rust-test/README.md
IGNORE_CASE=1 cargo run -- rUSt /Users/tengzhang/Documents/Core/rust-test/README.md

cargo test

*/
mod mini_web_server;
use mini_web_server::start_server;

#[async_std::main]
async fn main() {
    start_server().await;
}
