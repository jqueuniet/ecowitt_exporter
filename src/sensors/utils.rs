
pub fn temp_f_to_c(f: f64) -> f64 {
    round_to_x_places((f - 32.0) / 1.8, 1)
}

pub fn press_inhg_to_pa(inhg: f64) -> f64 {
    (inhg / 0.00029530).round()
}

pub fn speed_mph_to_kmph(mph: f64) -> f64 {
    round_to_x_places(mph * 1.60934, 2)
}

pub fn length_in_to_mm(inch: f64) -> f64 {
    round_to_x_places(inch * 25.4, 1)
}

// Round converted floats to a specific decimal place
// this avoids giving any false idea about metric accuracy
fn round_to_x_places(value: f64, power: u32) -> f64 {
    let base: i64 = 10;
    let multiplier: f64 = base.pow(power) as f64;
    f64::round(value * multiplier) / multiplier
}