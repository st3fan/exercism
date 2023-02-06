const PRODUCTION_RATE: f64 = 221.0;

fn success_rate_for_speed(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!("invalid speed: {}", speed),
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    speed as f64 * PRODUCTION_RATE * success_rate_for_speed(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate = production_rate_per_hour(speed) / 60.0;
    rate as u32
}
