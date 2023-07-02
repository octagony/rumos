use brightness::Brightness;
use futures::TryStreamExt;
use futures::executor::block_on;

fn main(){
    let future=show_brightness();
    block_on(future);
}

async fn show_brightness() -> Result<(), brightness::Error> {
    brightness::brightness_devices().try_for_each(|dev| async move {
        let name = dev.device_name().await?;
        let value = dev.get().await?;
        println!("Brightness of device {} is {}%", name, value);
        Ok(())
    }).await
}
