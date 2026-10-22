pub mod control_funcs {
    use crate::args::{Cli, SetArgs};
    use crate::config::{RumosConfig, apply_style};
    use crate::enums::BrightMode;
    use brightness::Brightness;
    use colored::*;
    use futures::{StreamExt, TryStreamExt};

    pub async fn set_brightness(args: &SetArgs, cli: &Cli) -> Result<(), brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if let Some(ref target) = cli.device {
                    if dev
                        .device_name()
                        .await
                        .unwrap_or_else(|_| "unknown".to_string())
                        != *target
                    {
                        return Ok(());
                    }
                }
                dev.set(args.level).await?;
                Ok(())
            })
            .await
    }

    pub async fn increase_or_decrease_brightness(
        args: &SetArgs,
        mode: &BrightMode,
        cli: &Cli,
    ) -> Result<(), brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if let Some(ref target) = cli.device {
                    if dev
                        .device_name()
                        .await
                        .unwrap_or_else(|_| "unknown".to_string())
                        != *target
                    {
                        return Ok(());
                    }
                }
                if let Ok(current) = dev.get().await {
                    let new_value = match mode {
                        BrightMode::Increase => (current + args.level).min(100),
                        BrightMode::Decrease => current.saturating_sub(args.level),
                    };
                    dev.set(new_value).await?;
                }
                Ok(())
            })
            .await
    }

    pub async fn print_brightness_level(
        cli: &Cli,
        config: &RumosConfig,
    ) -> Result<(), brightness::Error> {
        let mut stream = brightness::brightness_devices();
        let mut found = false;

        while let Some(result) = stream.next().await {
            let dev = match result {
                Ok(dev) => dev,
                Err(_) => continue,
            };

            if let Some(ref target) = cli.device {
                if dev
                    .device_name()
                    .await
                    .unwrap_or_else(|_| "unknown".to_string())
                    != *target
                {
                    continue;
                }
            }

            if let Ok(current) = dev.get().await {
                if !cli.quiet {
                    let name = dev
                        .device_name()
                        .await
                        .unwrap_or_else(|_| "unknown".to_string());

                    if cli.percent {
                        let styled_percent = apply_style(
                            &current.to_string(),
                            &config.percent_font_color,
                            &config.percent_font_style,
                        );
                        println!("{}", styled_percent);
                    } else {
                        let styled_name = apply_style(
                            &name,
                            &config.device_font_color,
                            &config.device_font_style,
                        );
                        let styled_percent = apply_style(
                            &current.to_string(),
                            &config.percent_font_color,
                            &config.percent_font_style,
                        );
                        println!("{}: {}", styled_name, styled_percent);
                    }
                }
                found = true;
            }
        }

        if !found && !cli.quiet {
            if let Some(ref target) = cli.device {
                eprintln!("Device '{}' not found", target.red().bold());
            } else {
                eprintln!("No brightness devices found");
            }
        }
        Ok(())
    }

    pub async fn list_devices() -> Result<(), brightness::Error> {
        let mut stream = brightness::brightness_devices();
        let mut found = false;

        while let Some(result) = stream.next().await {
            if let Ok(dev) = result {
                let name = dev
                    .device_name()
                    .await
                    .unwrap_or_else(|_| "unknown".to_string());
                let current = dev.get().await.unwrap_or(0);
                println!("-> {} (current: {})", name, current);
                found = true;
            }
        }

        if !found {
            println!("No brightness devices found.");
        }
        Ok(())
    }

    pub async fn set_filtered_devices(cli: &Cli, level: u32) -> Result<(), brightness::Error> {
        brightness::brightness_devices()
            .try_for_each(|mut dev| async move {
                if let Some(ref target) = cli.device {
                    if dev
                        .device_name()
                        .await
                        .unwrap_or_else(|_| "unknown".to_string())
                        != *target
                    {
                        return Ok(());
                    }
                }
                dev.set(level).await?;
                Ok(())
            })
            .await
    }
}
