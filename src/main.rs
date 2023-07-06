// Executor
use futures::executor;
// Args
mod args;
// Funcs
mod funcs;
// Setup
mod setup;
use setup::main_mod;

 fn main() {
    let _ = executor::block_on(main_mod::main_launch());
}
