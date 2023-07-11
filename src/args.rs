use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get the brightness level (in percent)
    Get,
    /// Set the brightness level (in percent)
    Set(SetArgs),
    /// Increase the brightness level (in percent)
    Inc(SetArgs),
    /// Decrease the brightness level (in percent)
    Dec(SetArgs),
    /// Set maximum brightness level
    Max,
    /// Set mininum brightness level
    Min

}

#[derive(Debug,Parser)]
pub struct SetArgs {
    #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
    pub percent: Option<u8>,
}