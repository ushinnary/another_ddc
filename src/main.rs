pub mod ddcutil;
pub mod macros;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Brightness value to apply to the monitor
    #[arg(short, long)]
    brightness: Option<u8>,

    /// Smoothly transition to the new brightness value
    #[arg(short, long)]
    smooth: bool,
}
fn main() {
    let args = Args::parse();

    if let (Some(brightness), smooth) = (args.brightness, args.smooth) {
        let inst = ddcutil::ScreenManagement::new();
        inst.set_brightness(brightness, smooth).unwrap();
    }
}
