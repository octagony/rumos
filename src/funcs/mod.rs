pub mod control_funcs {
    use crate::args::{Cli, SetArgs};
    use crate::constants::{MAX_SAFE_BRIGHTNESS, MIN_SAFE_BRIGHTNESS};
    use crate::enums::BrightMode;
    use brightness::Brightness;
    use colored::*;
    use futures::TryStreamExt;
    use std::process::ExitCode;

    pub async fn set_brightness(args: &SetArgs) -> Result<ExitCode, brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if args.percent < MIN_SAFE_BRIGHTNESS {
                    dev.set(MIN_SAFE_BRIGHTNESS).await?;
                    return Ok(());
                }
                let _ = dev.set(args.percent).await?;
                Ok(())
            })
            .await?;
        Ok(ExitCode::SUCCESS)
    }

    pub async fn increase_or_decrease_brightness(
        args: &SetArgs,
        mode: &BrightMode,
    ) -> Result<ExitCode, brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                let level = dev.get().await?;
                match mode {
                    BrightMode::Increase => {
                        if level < MAX_SAFE_BRIGHTNESS {
                            dev.set(level + args.percent).await?;
                        } else {
                            return Ok(());
                        }
                    }
                    BrightMode::Decrease => {
                        let calculate_value =
                            dev.get().await? < (args.percent + MIN_SAFE_BRIGHTNESS);
                        if calculate_value {
                            dev.set(MIN_SAFE_BRIGHTNESS).await?;
                        } else {
                            dev.set(level - args.percent).await?;
                        }
                    }
                }
                Ok(())
            })
            .await?;
        Ok(ExitCode::SUCCESS)
    }

    pub async fn print_brightness_level(cli: Cli) -> Result<ExitCode, brightness::Error> {
        let _ = brightness::brightness_devices()
            .try_for_each(|dev| async move {
                let (device, result) = (dev.device_name().await?, dev.get().await?);
                if !cli.quiet && !cli.percent {
                    if result >= MAX_SAFE_BRIGHTNESS {
                        println!(
                            "{} brightness level reached ({})",
                            "Maximum".green().bold(),
                            "100%".green().bold()
                        );
                        return Ok(());
                    }
                    if result <= MIN_SAFE_BRIGHTNESS {
                        println!(
                            "{} brightness level reached ({})",
                            "Minimum".red().bold(),
                            "5%".red().bold()
                        );
                        return Ok(());
                    }
                }
                if cli.quiet {
                    return Ok(());
                }
                if cli.percent {
                    println!("{}", format!("{result}%").yellow().bold());
                    return Ok(());
                }
                println!(
                    "Brightness of device {} is {}",
                    device.blue().bold(),
                    format!("{result}%").yellow().bold()
                );
                Ok(())
            })
            .await;
        Ok(ExitCode::SUCCESS)
    }
}
