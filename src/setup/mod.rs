pub mod main_mod{
    use clap::Parser;
    use futures::TryStreamExt;
    use crate::{args::{Cli, Commands}, funcs::control_funcs::{set_max_level, set_min_level}};
	use std::error::Error;
    use crate::funcs::control_funcs;

pub async fn main_launch()->Result<(), Box<dyn Error>>{
    let cli = Cli::parse();
    match &cli.command{
        Commands::Get=>{
            control_funcs::show_brightness().await?;
        }
        Commands::Set(percent) => {
            control_funcs::set_brightness(percent).await?;
        }
        Commands::Inc(percent) => {
            control_funcs::increase_or_decrease_brightness(percent, "inc").await?
        }
        Commands::Dec(percent) => {
            control_funcs::increase_or_decrease_brightness(percent, "dec").await?
        },
        Commands::Max=>{
            brightness::brightness_devices().try_for_each(|dev| async move {
                set_max_level(dev).await?;
                Ok(())
        }).await?
        }
        Commands::Min=>{
            brightness::brightness_devices().try_for_each(|dev| async move {
                set_min_level(dev).await?;
                Ok(())
        }).await?
        }
    }
    Ok(())
}
}

