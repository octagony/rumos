pub mod control_funcs {
    use crate::args::{Cli, SetArgs};
    use brightness::Brightness;
    use colored::*;
    use futures::TryStreamExt;
    use std::process::ExitCode;

    pub async fn set_brightness(args: &SetArgs) -> Result<ExitCode, brightness::Error> {
        let arg_percent = args.percent as u32;

        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if arg_percent < 5 {
                    dev.set(5).await?;
                    return Ok(());
                }
                let _ = dev.set(arg_percent).await?;
                Ok(())
            })
            .await?;
        Ok(ExitCode::SUCCESS)
    }

    pub async fn increase_or_decrease_brightness(
        args: &SetArgs,
        mode: &str,
    ) -> Result<ExitCode, brightness::Error> {
        let arg_percent = args.percent as u32;

        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                let level = dev.get().await?;
                if mode == "inc" {
                    if level < 100 {
                        dev.set(level + arg_percent).await?;
                    } else {
                        return Ok(());
                    }
                } else {
                    let calculate_value = dev.get().await? < (arg_percent + 5);
                    if calculate_value {
                        dev.set(5).await?;
                    } else {
                        dev.set(level - arg_percent).await?;
                    }
                };
                Ok(())
            })
            .await?;
        Ok(ExitCode::SUCCESS)
    }

    pub async fn print_brightness_lelel(cli: Cli) -> Result<ExitCode, brightness::Error> {
        let _ = brightness::brightness_devices()
            .try_for_each(|dev| async move {
                let (device, result) = (dev.device_name().await?, dev.get().await?);
                if !cli.quiet && !cli.percent {
                    if result >= 100 {
                        println!(
                            "{} brightness level reached ({})",
                            "Maximum".green().bold(),
                            "100%".green().bold()
                        );
                        return Ok(());
                    }
                    if result <= 5 {
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
