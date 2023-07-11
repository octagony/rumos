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
                let calcucate_value = |level,percent|->Result<u32,String>{  
                    if level - (percent as u32) < 5 {
                        return  Err(String::from("value"));
                    }
                    return Ok(percent)
                };
                let value = calcucate_value(level,percent.percent.unwrap() as u32);

               if value.is_ok(){
                println!("Value ok")
               }else{
                println!("Value not okey")
               }
                // if Ok(value) {
                //     // println!("Mininum brightness level is reached (5%)");
                //     dev.set().await?;
                // }else{
                //     // dev.set(level - percent.percent.unwrap() as u32).await?;
                // }
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

pub async fn set_max_level(mut dev:BrightnessDevice)->Result<(),brightness::Error>{
            dev.set(100).await?;
            print_brightness_lelel(dev).await?;
            Ok(())
}

pub async fn set_min_level(mut dev:BrightnessDevice)->Result<(),brightness::Error>{
            dev.set(5).await?;
            print_brightness_lelel(dev).await?;
            Ok(())
}
}

