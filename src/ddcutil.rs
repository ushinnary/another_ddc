use crate::helpers::ddc_brightness;
use ddc_hi::Display;

const MAX_BRIGHTNESS: u16 = 100;
const MIN_BRIGHTNESS: u16 = 0;
const SLEEP_DURATION: u64 = 50;

pub struct ScreenManagement {
    monitors: Vec<Display>,
    smooth: bool,
}

impl Default for ScreenManagement {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ScreenManagement {
    pub fn new(smooth: bool) -> Self {
        Self {
            smooth,
            monitors: Display::enumerate(),
        }
    }
    pub fn increase_brightness(&mut self) -> Result<(), String> {
        self.adjust_brightness(1)
    }

    pub fn decrease_brightness(&mut self) -> Result<(), String> {
        self.adjust_brightness(-1)
    }

    fn adjust_brightness(&mut self, adjustment: i16) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();
        let new_brightness = if adjustment > 0 {
            all_min_brightness
                .saturating_add(adjustment.unsigned_abs())
                .min(MAX_BRIGHTNESS)
        } else {
            all_min_brightness
                .saturating_sub(adjustment.unsigned_abs())
                .max(MIN_BRIGHTNESS)
        };

        self.set_brightness(new_brightness)
    }

    pub fn set_brightness(&mut self, brightness: u16) -> Result<(), String> {
        let brightness = brightness.min(100).max(0);

        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();

        match self.smooth {
            true => {
                let range =
                    ddc_brightness::get_range_for_brightness(brightness, all_min_brightness);

                for new_brightness in range {
                    self.apply_brightness_to_all_monitors(new_brightness)?;

                    std::thread::sleep(std::time::Duration::from_millis(SLEEP_DURATION));
                }
            }
            false => {
                self.apply_brightness_to_all_monitors(brightness)?;
            }
        }

        Ok(())
    }

    fn apply_brightness_to_all_monitors(&mut self, brightness: u16) -> Result<(), String> {
        for monitor in &mut self.monitors {
            ddc_brightness::set_monitor_brightness(monitor, brightness)
                .map_err(|e| format!("Failed to set brightness: {}", e))?;
        }

        Ok(())
    }

    fn get_all_monitors_min_and_max_brightness(&mut self) -> (u16, u16) {
        self.monitors
            .iter_mut()
            .filter_map(|monitor| ddc_brightness::get_monitor_brightness(monitor).ok())
            .fold(
                (MAX_BRIGHTNESS, MIN_BRIGHTNESS),
                |(min_brightness, max_brightness), (current_brightness, _)| {
                    (
                        min_brightness.min(current_brightness),
                        max_brightness.max(current_brightness),
                    )
                },
            )
    }
}
