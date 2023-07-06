pub mod control_funcs{
	use brightness::Brightness;
	use futures::TryStreamExt;
	use crate::args::SetArgs;

pub async fn show_brightness() -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|dev| async move {
			let name = dev.device_name().await?;
			let value = dev.get().await?;
			println!("Brightness of device {} is {}%", name, value);
			Ok(())
	}).await
} 
	
pub async fn set_brightness(percent:&SetArgs) -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|mut dev| async move {
			let _ = dev.set(percent.percent.unwrap() as u32).await?;
			let (device, level) = (dev.device_name().await?, dev.get().await?);
			println!("Brightness of device {} is {}%", device, level);
			Ok(())
	}).await
}

pub async fn increase_or_decrease_brightness(percent:&SetArgs, com:&str ) -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|mut dev| async move {
			let level = dev.get().await?;
            if com == "inc" {
                if level < 100{
                  dev.set(level + percent.percent.unwrap() as u32).await?
                }
            }else{
                dev.set(level - percent.percent.unwrap() as u32).await?
            };
            let (device,result) = (dev.device_name().await?,dev.get().await?);
			println!("Brightness of device {} is {}%",device, result);
			Ok(())
	}).await
}


}
