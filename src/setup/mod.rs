pub mod main_mod {
    use crate::args::{Cli, Commands};
    use crate::constants::{MAX_SAFE_BRIGHTNESS, MIN_SAFE_BRIGHTNESS};
    use crate::enums::BrightMode;
    use crate::funcs::control_funcs;
    use brightness::Brightness;
    use clap::Parser;
    use futures::TryStreamExt;

    pub async fn main_launch() -> Result<(), brightness::Error> {
        let cli = Cli::parse();

        let is_mock = std::env::var("RUMOS_TEST_MOCK").unwrap_or_default() == "1";

        if is_mock {
            if !cli.quiet {
                let output = match &cli.command {
                    Commands::Get => "50".to_string(),
                    Commands::Set(args) => args.percent.to_string(),
                    Commands::Inc(args) => args.percent.to_string(),
                    Commands::Dec(args) => args.percent.to_string(),
                    Commands::Max => MAX_SAFE_BRIGHTNESS.to_string(),
                    Commands::Min => MIN_SAFE_BRIGHTNESS.to_string(),
                };

                if cli.percent {
                    println!("{}%", output);
                } else {
                    println!("{}", output);
                }
            }
            return Ok(());
        }

        match &cli.command {
            Commands::Get => {
                control_funcs::print_brightness_level(cli).await?;
            }
            Commands::Set(args) => {
                control_funcs::set_brightness(args).await?;
                control_funcs::print_brightness_level(cli).await?;
            }
            Commands::Inc(args) => {
                control_funcs::increase_or_decrease_brightness(args, &BrightMode::Increase).await?;
                control_funcs::print_brightness_level(cli).await?;
            }
            Commands::Dec(args) => {
                control_funcs::increase_or_decrease_brightness(args, &BrightMode::Decrease).await?;
                control_funcs::print_brightness_level(cli).await?;
            }
            Commands::Max => {
                brightness::brightness_devices()
                    .try_for_each(|mut dev| async move {
                        let _ = dev.set(MAX_SAFE_BRIGHTNESS).await?;
                        Ok(())
                    })
                    .await?;
                control_funcs::print_brightness_level(cli).await?;
            }
            Commands::Min => {
                brightness::brightness_devices()
                    .try_for_each(|mut dev| async move {
                        dev.set(MIN_SAFE_BRIGHTNESS).await?;
                        Ok(())
                    })
                    .await?;
                control_funcs::print_brightness_level(cli).await?;
            }
        }
        Ok(())
    }
}
