pub mod main_mod {
    use crate::args::{Cli, Commands};
    use crate::funcs::control_funcs;
    use brightness::Brightness;
    use clap::Parser;
    use futures::TryStreamExt;
    use std::process::ExitCode;

    pub async fn main_launch() -> Result<(), brightness::Error> {
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
                let _ = brightness::brightness_devices()
                    .try_for_each(|mut dev| async move {
                        let _ = dev.set(100).await?;
                        Ok(())
                    })
                    .await;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Min => {
                brightness::brightness_devices()
                    .try_for_each(|mut dev| async move {
                        dev.set(5).await?;
                        Ok(())
                    })
                    .await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
        }
        Ok(())
    }
}
