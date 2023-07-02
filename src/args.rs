use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub struct RumosArgs {
    #[arg(short = "s", long = "set")]
    set: u8,
}
