

/// Initialize logger.
pub fn init_logger() {
    use std::io::Write;
    use env_logger::Builder;
    Builder::from_default_env()
        .format(|buf, record| {
            //let level = record.level();
            let message = record.args();
            writeln!(buf, "{}", message)
        })
        .init();
}

/// Initialize environment variables to default values.
pub fn init_env_var() {
    use std::env;
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info");
    //env::set_var("ICED_BACKEND", "tiny-skia");
    //env::set_var("WGPU_BACKEND", "VULKAN");
}

/// Calculate the angle (in radians) from the start point to the end point.
use iced::Point;
pub fn calculate_angle(start: Point, end: Point) -> f64 {
    let dx = (end.x - start.x) as f64;
    let dy = (end.y - start.y) as f64;
    let angle = dy.atan2(dx);
    angle
}

pub fn calculate_angle_degrees(start: Point, end: Point) -> f64 {
    let radians = calculate_angle(start, end);
    let degrees = radians_to_degrees(radians);
    degrees
}

/// Converts an angle from radians to degrees.
pub fn radians_to_degrees(radians: f64) -> f64 {
    use std::f64::consts::PI;
    radians * 180.0 / PI
}