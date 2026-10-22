// Constants
mod constants;
// Executor
// Args
mod args;
// Funcs
mod funcs;
// Args
mod enums;
// Setup
mod setup;
// Config
mod config;

#[tokio::main]
async fn main() {
    if let Err(e) = setup::main_mod::main_launch().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
