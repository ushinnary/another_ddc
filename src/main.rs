pub mod ddcutil;
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Brightness value to apply to the monitor
    #[arg(short, long)]
    brightness: Option<u16>,

    /// Smoothly transition to the new brightness value
    #[arg(short, long)]
    smooth: bool,

    /// Increase the brightness by one
    #[arg(short, long)]
    increase: bool,

    /// Decrease the brightness by one
    #[arg(short, long)]
    decrease: bool,
}
fn main() {
    let args = Args::parse();

    match (args.brightness, args.decrease, args.increase) {
        (Some(brightness), false, false) => {
            let mut ddc_management = ddcutil::ScreenManagement::new(args.smooth);
            ddc_management.set_brightness(brightness).unwrap();
        }
        (_, true, false) => {
            let mut ddc_management = ddcutil::ScreenManagement::new(false);
            ddc_management.decrease_brightness().unwrap();
        }
        (_, false, true) => {
            let mut ddc_management = ddcutil::ScreenManagement::new(false);
            ddc_management.increase_brightness().unwrap();
        }
        _ => (),
    }
}
