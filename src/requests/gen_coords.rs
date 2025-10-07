pub fn gen_coords() -> (f64, f64) {
    let lat: f64 = 38.366966;
    let lon: f64 = 27.202704;
    // Add random noise to the coordinates 0.000001 to 0.000009
    let noise_lat: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let noise_lon: f64 = (rand::random::<u32>() % 9 + 1) as f64 * 0.000001;
    let lat = lat + noise_lat;
    let lon = lon + noise_lon;
    (round_six(lat), round_six(lon))
}
fn round_six(x: f64) -> f64 {
    (x * 1_000_000.0).round() / 1_000_000.0
}