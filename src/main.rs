use std::error::Error;
use clap::{Args, Parser, Subcommand};
use brightness::Brightness;
use futures::TryStreamExt;
use futures::executor;
mod args;
use args::{Cli,Commands,SetArgs,};

async fn start()->Result<(), Box<dyn Error>>{
    let cli = Cli::parse();
    match &cli.command{
        Commands::Get=>{
            show_brightness().await?;
        }
        Commands::Set(percent) => {
           control_brightness(percent).await?;
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

async fn control_brightness(percent:&SetArgs) -> Result<(), brightness::Error> {
    brightness::brightness_devices().try_for_each(|mut dev| async move {
        let _ = dev.set(percent.percent.unwrap() as u32).await?;
        let (device, level) = (dev.device_name().await?, dev.get().await?);
        println!("Brightness of device {} is {}%", device, level);
        Ok(())
    }).await
}


async fn show_brightness() -> Result<(), brightness::Error> {
    brightness::brightness_devices().try_for_each(|dev| async move {
        let name = dev.device_name().await?;
        let value = dev.get().await?;
        println!("Brightness of device {} is {}%", name, value);
        Ok(())
    }).await
} 


