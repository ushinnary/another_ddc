pub struct Helper;
impl Helper {
    pub fn get_range_for_brightness(brightness: u16, all_min_brightness: u16) -> Vec<u16> {
        if brightness > all_min_brightness {
            (all_min_brightness..=brightness).collect()
        } else {
            (brightness..=all_min_brightness).rev().collect()
        }
    }
}
