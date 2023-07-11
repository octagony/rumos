pub mod main_mod{
    use clap::Parser;
    use futures::TryStreamExt;
    use crate::{args::{Cli, Commands, SetArgs}, funcs::control_funcs::{set_max_level, set_min_level}};
	use std::{error::Error};
    use crate::funcs::control_funcs;

fn validate_percent (percent:&SetArgs)->&SetArgs{
    println!("Start Validating");
    if Some(percent.percent) < Some(Some(u8::MIN)) && Some(percent.percent) > Some(Some(u8::MAX)){
        println!("Value must be 0 or 100");
        return &SetArgs{percent:{Some(0)}}
    }else{
        return percent
    }
}

pub async fn main_launch()->Result<(), Box<dyn Error>>{
    let cli = Cli::parse();
    match &cli.command{
        Commands::Get=>{
            control_funcs::show_brightness().await?;
        }
        Commands::Set(percent) => {
            let needed_percent = validate_percent(percent);
            control_funcs::set_brightness(needed_percent).await?;
        }
        Commands::Inc(percent) => {
            let needed_percent = validate_percent(percent);
            control_funcs::increase_or_decrease_brightness(needed_percent, "inc").await?
        }
        Commands::Dec(percent) => {
            let needed_percent = validate_percent(percent);
            control_funcs::increase_or_decrease_brightness(needed_percent, "dec").await?
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

