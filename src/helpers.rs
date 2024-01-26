pub mod ddc_brightness {
    use ddc_hi::{traits::Ddc, Display};

    pub fn get_range_for_brightness(brightness: u16, all_min_brightness: u16) -> Vec<u16> {
        if brightness > all_min_brightness {
            (all_min_brightness..=brightness).collect()
        } else {
            (brightness..=all_min_brightness).rev().collect()
        }
    }
    pub fn set_monitor_brightness(monitor: &mut Display, brightness: u16) -> Result<(), String> {
        monitor
            .handle
            .set_vcp_feature(0x10, brightness)
            .expect("Couldn't set brightness value");

        Ok(())
    }

    pub fn get_monitor_brightness(monitor: &mut Display) -> Result<(u16, u16), String> {
        let brightness = monitor
            .handle
            .get_vcp_feature(0x10)
            .expect("Brightness value get failed");

        Ok((brightness.value(), brightness.maximum()))
    }
}
