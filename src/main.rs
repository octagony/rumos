use brightness::{Brightness, BrightnessDevice};
use futures::TryStreamExt;
use futures::executor::block_on;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct RumosArgs {
    #[arg(short,long)]
    set: Option<String>,
    #[arg(short,long)]
    increase: Option<String>,
    #[arg(short,long)]
    decrease: Option<String>,
}

fn main(){
    let args = RumosArgs::parse();
    println!("{:?}",args);
    let future=control_brightness();
    let _ = block_on(future);
}

// async fn show_brightness() -> Result<(), brightness::Error> {
//     brightness::brightness_devices().try_for_each(|dev| async move {
//         let name = dev.device_name().await?;
//         let value = dev.get().await?;
//         println!("Brightness of device {} is {}%", name, value);
//         Ok(())
//     }).await
// }

async fn control_brightness() -> Result<(), brightness::Error> {
    brightness::brightness_devices().try_for_each(|mut dev| async move {
        let _ = dev.set(50);
        Ok(())
    }).await
}
