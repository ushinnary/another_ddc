use ddc_hi::{traits::Ddc, Display};

pub struct ScreenManagement {
    pub monitors: Box<Vec<Display>>,
    smooth: bool,
}

impl Default for ScreenManagement {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ScreenManagement {
    pub fn new(smooth: bool) -> Self {
        let monitors: Vec<Display> = Display::enumerate();

        Self {
            monitors: Box::new(monitors),
            smooth,
        }
    }

    pub fn increase_brightness(&mut self) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();

        let new_brightness = (all_min_brightness + 1).min(100);

        self.set_brightness(new_brightness).unwrap();

        Ok(())
    }

    pub fn decrease_brightness(&mut self) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();

        let new_brightness = (all_min_brightness - 1).max(0);

        self.set_brightness(new_brightness).unwrap();

        Ok(())
    }

    // TODO: Implement smooth transitions for multiple monitors
    pub fn set_brightness(&mut self, brightness: u16) -> Result<(), String> {
        // let mut handles: Vec<std::thread::JoinHandle<std::result::Result<(), String>>> = Vec::new();

        // let (all_min_brightness, all_max_brightness) =
        // self.get_all_monitors_min_and_max_brightness();
        // dbg!(all_min_brightness);
        // dbg!(all_max_brightness);

        // let barrier = Arc::new(Barrier::new(self.monitors.len()));

        self.monitors.iter_mut().for_each(|monitor| {
            // let brightness_clone = brightness;
            // let smooth_clone = smooth;
            // let barrier_clone = Arc::clone(&barrier);

            // let handle = thread::spawn(move || {
            //     let (current_brightness, max_brightness) = Self::get_ddcutil_brightness(&monitor)?;

            //     let brightness = brightness_clone.min(max_brightness).max(0);

            //     match smooth_clone {
            //         true => {
            //             let to_increment = brightness > current_brightness;

            //             let range: Vec<u16> = if brightness > current_brightness {
            //                 (current_brightness..=brightness).collect()
            //             } else {
            //                 (brightness..=current_brightness.min(max_brightness))
            //                     .rev()
            //                     .collect()
            //             };

            //             for new_brightness in range {
            //                 barrier_clone.wait(); // Wait for all threads to reach this point
            //                 dbg!(to_increment, new_brightness);

            //                 if to_increment && new_brightness > current_brightness
            //                     || !to_increment && new_brightness < current_brightness
            //                 {
            //                     Self::set_ddcutil_brightness(&monitor, new_brightness)?;
            //                 }

            //                 std::thread::sleep(std::time::Duration::from_millis(50));
            //             }
            //         }
            //         false => {
            //             Self::set_ddcutil_brightness(&monitor, brightness)?;
            //         }
            //     }

            //     Ok(())
            // });

            // handles.push(handle);
            Self::set_ddcutil_brightness(monitor, brightness).unwrap();
        });

        // for handle in handles {
        //     handle.join().unwrap()?;
        // }

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
        self.monitors
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
