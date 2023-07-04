pub mod main_mod{
    use clap::Parser;
    use crate::args::{Cli, Commands};
	use std::error::Error;
    use crate::funcs::control_funcs;

pub async fn main_launch()->Result<(), Box<dyn Error>>{
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
}

