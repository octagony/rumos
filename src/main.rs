use clap::{Args, Parser, Subcommand};
use brightness::Brightness;
use futures::TryStreamExt;
use futures::executor;
mod args;
use args::{Cli,Commands,SetArgs,};



fn main() {
    let cli = Cli::parse();
    match &cli.command{
        Commands::Set(percent) => {
            println!("Set the percentage of brightness to {:?}%", percent.percent);
        }
        Commands::Inc(percent) => {
            println!("Increase the percentage of brightness to {:?}%", percent.percent);
        }
        Commands::Dec(percent) => {
            println!("Decrease the percentage of brightness to {:?}%", percent.percent);
        }
    }
}


async fn show_brightness() -> Result<(), brightness::Error> {
    brightness::brightness_devices().try_for_each(|dev| async move {
        let name = dev.device_name().await?;
        let value = dev.get().await?;
        println!("Brightness of device {} is {}%", name, value);
        Ok(())
    }).await
} 

async fn control_brightness() -> Result<(), brightness::Error> {

    brightness::brightness_devices().try_for_each(|mut dev| async move {
        let _ = dev.set(90).await?;
        let (device, level) = (dev.device_name().await?, dev.get().await?);
        println!("Brightness of device {} is {}%", device, level);
        Ok(())
    }).await
}
