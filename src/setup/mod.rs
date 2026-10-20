pub mod main_mod {
    use crate::args::{Cli, Commands};
    use crate::constants::{MAX_SAFE_BRIGHTNESS, MIN_SAFE_BRIGHTNESS};
    use crate::enums::BrightMode;
    use crate::funcs::control_funcs;
    use clap::Parser;

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
                    Commands::List => "
                    Mock devices:\n
                    - intel_backlight (50)\n
                    - nvidia (50)"
                        .to_string(),
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
            Commands::List => {
                control_funcs::list_devices().await?;
            }
            Commands::Get => {
                control_funcs::print_brightness_level(&cli).await?;
            }
            Commands::Set(args) => {
                control_funcs::set_brightness(args, &cli).await?;
                control_funcs::print_brightness_level(&cli).await?;
            }
            Commands::Inc(args) => {
                control_funcs::increase_or_decrease_brightness(args, &BrightMode::Increase, &cli)
                    .await?;
                control_funcs::print_brightness_level(&cli).await?;
            }
            Commands::Dec(args) => {
                control_funcs::increase_or_decrease_brightness(args, &BrightMode::Decrease, &cli)
                    .await?;
                control_funcs::print_brightness_level(&cli).await?;
            }
            Commands::Max => {
                control_funcs::set_filtered_devices(&cli, MAX_SAFE_BRIGHTNESS).await?;
                control_funcs::print_brightness_level(&cli).await?;
            }
            Commands::Min => {
                control_funcs::set_filtered_devices(&cli, MIN_SAFE_BRIGHTNESS).await?;
                control_funcs::print_brightness_level(&cli).await?;
            }
        }
        Ok(())
    }
}
