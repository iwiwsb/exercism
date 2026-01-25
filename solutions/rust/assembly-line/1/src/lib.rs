pub fn production_rate_per_hour(speed: u8) -> f64 {
    speed as f64
        * 221.0
        * match speed {
            5..=8 => 0.9,
            9 | 10 => 0.77,
            _ => 1.0,
        }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
