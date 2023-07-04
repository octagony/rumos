pub mod control_funcs{
	use brightness::Brightness;
	use futures::TryStreamExt;
	use crate::args::SetArgs;
	
pub async fn control_brightness(percent:&SetArgs) -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|mut dev| async move {
			let _ = dev.set(percent.percent.unwrap() as u32).await?;
			let (device, level) = (dev.device_name().await?, dev.get().await?);
			println!("Brightness of device {} is {}%", device, level);
			Ok(())
	}).await
}


pub async fn show_brightness() -> Result<(), brightness::Error> {
	brightness::brightness_devices().try_for_each(|dev| async move {
			let name = dev.device_name().await?;
			let value = dev.get().await?;
			println!("Brightness of device {} is {}%", name, value);
			Ok(())
	}).await
} 
}
