use std::error::Error;
use clap::Parser;
use futures::executor;
// Args
mod args;
use args::{Cli,Commands};
// Funcs
mod funcs;
use funcs::control_funcs;

async fn start()->Result<(), Box<dyn Error>>{
    let cli = Cli::parse();
    match &cli.command{
        Commands::Get=>{
           control_funcs::show_brightness().await?;
        }
        Commands::Set(percent) => {
            control_funcs::control_brightness(percent).await?;
        }
        Commands::Inc(percent) => {
            println!("Increase the percentage of brightness to {:?}%", percent.percent);
        }
        Commands::Dec(percent) => {
            println!("Decrease the percentage of brightness to {:?}%", percent.percent);
        }
    }
    Ok(())
}

 fn main() {
    let _ = executor::block_on(start());
}