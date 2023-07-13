pub mod main_mod {
    use crate::funcs::control_funcs;
    use crate::{
        args::{Cli, Commands},
        funcs::control_funcs::{set_max_level, set_min_level},
    };
    use clap::Parser;
    use futures::TryStreamExt;
    use std::error::Error;

    pub async fn main_launch() -> Result<(), Box<dyn Error>> {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Get => {
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Set(percent) => {
                control_funcs::set_brightness(percent).await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Inc(percent) => {
                control_funcs::increase_or_decrease_brightness(percent, "inc").await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Dec(percent) => {
                control_funcs::increase_or_decrease_brightness(percent, "dec").await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Max => {
                brightness::brightness_devices()
                    .try_for_each(|dev| async move {
                        set_max_level(dev).await?;
                        Ok(())
                    })
                    .await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Min => {
                brightness::brightness_devices()
                    .try_for_each(|dev| async move {
                        set_min_level(dev).await?;
                        Ok(())
                    })
                    .await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
        }
        Ok(())
    }
}
