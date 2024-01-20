use ddc_hi::{traits::Ddc, Display};

pub struct ScreenManagement {
    smooth: bool,
}

impl Default for ScreenManagement {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ScreenManagement {
    pub fn new(smooth: bool) -> Self {
        Self { smooth }
    }

    pub fn increase_brightness(&mut self) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();
        let new_brightness = all_min_brightness.saturating_add(1).min(100);

        self.set_brightness(new_brightness).unwrap();
        Ok(())
    }

    pub fn decrease_brightness(&mut self) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();
        let new_brightness = all_min_brightness.saturating_sub(1).max(0);

        self.set_brightness(new_brightness).unwrap();
        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: u16) -> Result<(), String> {
        let brightness = brightness.min(100).max(0);
        let mut monitors = Display::enumerate();

        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();

        match self.smooth {
            true => {
                let to_increment = brightness > all_min_brightness;
                let range: Vec<u16> = if to_increment {
                    (all_min_brightness..=brightness).collect()
                } else {
                    (brightness..=all_min_brightness).rev().collect()
                };

                for new_brightness in range {
                    monitors.iter_mut().for_each(|monitor| {
                        Self::set_ddcutil_brightness(monitor, new_brightness).unwrap();
                    });

                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
            false => {
                monitors.iter_mut().for_each(|monitor| {
                    Self::set_ddcutil_brightness(monitor, brightness).unwrap();
                });
            }
        }

        Ok(())
    }

    fn set_ddcutil_brightness(monitor: &mut Display, brightness: u16) -> Result<(), String> {
        monitor
            .handle
            .set_vcp_feature(0x10, brightness)
            .expect("Couldn't set brightness value");

        Ok(())
    }

    fn get_ddcutil_brightness(monitor: &mut Display) -> Result<(u16, u16), String> {
        let brightness = monitor
            .handle
            .get_vcp_feature(0x10)
            .expect("Brightness value get failed");

        Ok((brightness.value(), brightness.maximum()))
    }

    fn get_all_monitors_min_and_max_brightness(&mut self) -> (u16, u16) {
        Display::enumerate()
            .iter_mut()
            .map(|monitor| Self::get_ddcutil_brightness(monitor).unwrap())
            .fold(
                (255, 0),
                |(min_brightness, max_brightness), (current_brightness, _)| {
                    (
                        min_brightness.min(current_brightness),
                        max_brightness.max(current_brightness),
                    )
                },
            )
    }
}
