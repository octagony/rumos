pub mod main_mod {
    use crate::args::{Cli, Commands};
    use crate::funcs::control_funcs;
    use brightness::Brightness;
    use clap::Parser;
    use futures::TryStreamExt;

    pub async fn main_launch() -> Result<(), brightness::Error> {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Get => {
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Set(args) => {
                control_funcs::set_brightness(args).await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Inc(args) => {
                control_funcs::increase_or_decrease_brightness(args, "inc").await?;
                control_funcs::print_brightness_lelel(cli).await?;
            }
            Commands::Dec(args) => {
                control_funcs::increase_or_decrease_brightness(args, "dec").await?;
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
