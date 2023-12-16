use crate::run_ddcutil_command;
use std::{
    sync::{Arc, Barrier},
    thread,
};
pub struct ScreenManagement {
    pub monitor_ids: Vec<u8>,
}

impl Default for ScreenManagement {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenManagement {
    pub fn new() -> Self {
        let monitor_ids = Self::get_ddcutil_available_ids();
        Self { monitor_ids }
    }

    pub fn set_brightness(&self, brightness: u8, smooth: bool) -> Result<(), String> {
        let mut handles: Vec<std::thread::JoinHandle<std::result::Result<(), String>>> = Vec::new();
        let (all_min_brightness, all_max_brightness) =
            self.get_all_monitors_min_and_max_brightness();
        dbg!(all_min_brightness);
        dbg!(all_max_brightness);

        let barrier = Arc::new(Barrier::new(self.monitor_ids.len()));

        for monitor_id in &self.monitor_ids {
            let monitor_id_clone = *monitor_id;
            let brightness_clone = brightness;
            let smooth_clone = smooth;
            let barrier_clone = Arc::clone(&barrier);

            let handle = thread::spawn(move || {
                let (current_brightness, max_brightness) =
                    Self::get_ddcutil_brightness(monitor_id_clone)?;

                let brightness = brightness_clone.min(max_brightness).max(0);

                match smooth_clone {
                    true => {
                        let to_increment = brightness > current_brightness;

                        let range: Vec<u8> = if brightness > current_brightness {
                            (current_brightness..=brightness).collect()
                        } else {
                            (brightness..=current_brightness.min(max_brightness))
                                .rev()
                                .collect()
                        };

                        for new_brightness in range {
                            barrier_clone.wait(); // Wait for all threads to reach this point
                            dbg!(to_increment, new_brightness);

                            if to_increment && new_brightness > current_brightness
                                || !to_increment && new_brightness < current_brightness
                            {
                                Self::set_ddcutil_brightness(monitor_id_clone, new_brightness)?;
                            }

                            std::thread::sleep(std::time::Duration::from_millis(50));
                        }
                    }
                    false => {
                        Self::set_ddcutil_brightness(monitor_id_clone, brightness)?;
                    }
                }

                Ok(())
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()?;
        }

        Ok(())
    }

    pub fn get_brightness(&self) -> Result<Vec<(u8, u8)>, String> {
        let mut brightness = Vec::new();
        for monitor_id in &self.monitor_ids {
            brightness.push(Self::get_ddcutil_brightness(*monitor_id)?);
        }
        Ok(brightness)
    }

    fn get_ddcutil_available_ids() -> Vec<u8> {
        let stdout = run_ddcutil_command!("detect");

        stdout
            .lines()
            .filter(|line| line.contains("I2C bus"))
            .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    }

    fn set_ddcutil_brightness(monitor_id: u8, brightness: u8) -> Result<(), String> {
        run_ddcutil_command!(
            format!("--bus={}", monitor_id),
            "setvcp",
            "10",
            format!("{}", brightness)
        );

        Ok(())
    }

    fn get_ddcutil_brightness(monitor_id: u8) -> Result<(u8, u8), String> {
        let stdout = run_ddcutil_command!(format!("--bus={}", monitor_id), "getvcp", "10");
        // println!("stdout: {}", stdout);
        let brightness: Vec<u8> = stdout
            .split("value =")
            .enumerate()
            .filter(|(idx, _)| *idx > 0)
            .map(|(_, line)| {
                let values = line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();

                values.parse::<u8>().unwrap()
            })
            .collect();

        assert!(brightness.len() == 2);
        Ok((brightness[0], brightness[1]))
    }

    fn get_all_monitors_min_and_max_brightness(&self) -> (u8, u8) {
        self.monitor_ids
            .iter()
            .map(|monitor_id| Self::get_ddcutil_brightness(*monitor_id).unwrap())
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
