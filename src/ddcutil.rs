use crate::helpers::ddc_brightness;
use ddc_hi::{Backend, Display};

const MAX_BRIGHTNESS: u16 = 100;
const MIN_BRIGHTNESS: u16 = 0;
const SLEEP_DURATION: u64 = 50;

pub struct ScreenManagement {
    monitors: Vec<Display>,
    smooth: bool,
    step: u8,
}

impl Default for ScreenManagement {
    fn default() -> Self {
        Self::new(false, 1)
    }
}

impl ScreenManagement {
    pub fn new(smooth: bool, step: u8) -> Self {
        let mut monitors = Display::enumerate();

        // Using only NVAPI on Windows if available
        if cfg!(target_os = "windows") && Backend::values().contains(&Backend::Nvapi) {
            monitors.retain(|monitor| monitor.backend().name() == "nvapi");
        }

        Self {
            smooth,
            monitors,
            step: step.max(1),
        }
    }

    /// Increase the brightness of all monitors by one step.
    pub fn increase_brightness(&mut self) -> Result<(), String> {
        self.adjust_brightness(1)
    }

    /// Decrease the brightness of all monitors by one step.
    pub fn decrease_brightness(&mut self) -> Result<(), String> {
        self.adjust_brightness(-1)
    }

    /// Adjust the brightness of all monitors by the given value.
    fn adjust_brightness(&mut self, adjustment: i16) -> Result<(), String> {
        let (all_min_brightness, _) = self.get_all_monitors_min_and_max_brightness();
        let adjustment = adjustment * self.step as i16;

        let new_brightness = if adjustment > 0 {
            all_min_brightness.saturating_add(adjustment.unsigned_abs())
        } else {
            all_min_brightness.saturating_sub(adjustment.unsigned_abs())
        };

        self.set_brightness(new_brightness)
    }

    /// Set the brightness of all monitors to the given value.
    pub fn set_brightness(&mut self, brightness: u16) -> Result<(), String> {
        let brightness = brightness.min(MAX_BRIGHTNESS).max(MIN_BRIGHTNESS);

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

    /// Apply the given brightness value to all monitors.
    fn apply_brightness_to_all_monitors(&mut self, brightness: u16) -> Result<(), String> {
        for monitor in &mut self.monitors {
            ddc_brightness::set_monitor_brightness(monitor, brightness)
                .map_err(|e| format!("Failed to set brightness: {}", e))?;
        }

        Ok(())
    }

    /// Get the minimum and maximum brightness of all monitors.
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
