// Constants
mod constants;
// Executor
use futures::executor;
// Args
mod args;
// Funcs
mod funcs;
// Args
mod enums;
// Setup
mod setup;

fn main() {
    if let Err(e) = executor::block_on(setup::main_mod::main_launch()) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
