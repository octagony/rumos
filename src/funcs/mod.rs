pub mod control_funcs{
	use brightness::{Brightness, BrightnessDevice};
    use futures::TryStreamExt;
	use crate::args::SetArgs;

pub async fn show_brightness() -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|dev| async move {
            print_brightness_lelel(dev).await?;
			Ok(())
	}).await
} 
	
pub async fn set_brightness(percent:&SetArgs) -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|mut dev| async move {
			let _ = dev.set(percent.percent.unwrap() as u32).await?;
            print_brightness_lelel(dev).await?;
			Ok(())
	}).await
}

pub async fn increase_or_decrease_brightness(percent:&SetArgs, com:&str ) -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|mut dev| async move {
			let level = dev.get().await?;
            if com == "inc" {
                if level < 100{
                  dev.set(level + percent.percent.unwrap() as u32).await?;
                }else{
                    println!("Maximum brightness level is reached (100%)");
                    return Ok(())
                }
            }else {
                if percent.percent.unwrap() > 100{
                    set_min_level(dev).await?;
                    println!("Minimum brightness level is reached (5%)");
                    return Ok(())
                }else{
                  dev.set(level - percent.percent.unwrap() as u32).await?;
                }
            };
            print_brightness_lelel(dev).await?;
			Ok(())
	}).await
}

async fn print_brightness_lelel(dev:BrightnessDevice)->Result<(),brightness::Error>{
            let (device,result) = (dev.device_name().await?,dev.get().await?);
            println!("Brightness of device {} is {}%", device, result);
            Ok(())
}

async fn set_max_level(mut dev:BrightnessDevice)->Result<(),brightness::Error>{
            dev.set(100).await?;
            Ok(())
}

async fn set_min_level(mut dev:BrightnessDevice)->Result<(),brightness::Error>{
            dev.set(5).await?;
            Ok(())
}
}

