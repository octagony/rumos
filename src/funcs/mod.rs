pub mod control_funcs {
    use crate::args::{Cli, SetArgs};
    use brightness::Brightness;
    use colored::*;
    use futures::TryStreamExt;
    use std::process::ExitCode;
    use crate::enums::BrightMode;

    pub async fn set_brightness(args: &SetArgs) -> Result<ExitCode, brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if args.percent < 5 {
                    dev.set(5).await?;
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

        brightness::brightness_devices().try_for_each(|mut dev| async move{
            let level = dev.get().await?;
            match mode{
                BrightMode::Increase=>{
                    if level < 100 {
                        dev.set(level+args.percent).await?;
                    }else{
                        return Ok(())
                    }
                }
                    BrightMode::Decrease=>{
                    let calculate_value = dev.get().await? < (args.percent + 5);
                    if calculate_value {
                        dev.set(5).await?;
                    } else {
                        dev.set(level - args.percent).await?;
                    }
                    }
            }
            Ok(())
        }).await?;
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
