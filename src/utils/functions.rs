

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

/// Normalises a point to have a magnitude of 1
pub fn normalize_point(point: Point) -> Point {
    let length = (point.x.powi(2) + point.y.powi(2)).sqrt();
    if length != 0.0 {
        Point::new(point.x / length, point.y / length)
    } else {
        Point::new(0.0, 0.0)
    }
}


/// Multiply a point by a scalar
pub fn multiply_point(point: Point, scalar: f32) -> Point {
    Point::new(point.x * scalar, point.y * scalar)
}

/// Add 2 points
pub fn add_point(a: Point, b: Point) -> Point {
    Point::new(a.x + b.x, a.y + b.y)
}

/// Invert point
pub fn invert_point(point: Point) -> Point {
    Point::new(-point.x, -point.y)
}

/// Sets the SVG fill color
pub fn set_svg_fill(svg_bytes: &[u8], color: String) -> &'static [u8] {
    let mut svg = String::from_utf8_lossy(svg_bytes).to_string();

    // Modify the fill attribute of the path
    if let Some(start) = svg.find("<path") {
        if let Some(end) = svg[start..].find("/>") {
            let end_index = start + end;
            svg.insert_str(end_index, &format!(r#" fill="{}""#, color).as_str());
        }
    }

    // leak memory by storing the svg in memory
    Box::leak(svg.into_bytes().into_boxed_slice())
}


/// Computes the Gaussian Probability Density Function between two points.
/// a larger sigma will increase the range of the function
/// a smaller sigma will decrease the range of the function
/// The returned value is normalized to be in the range 0-1.0.
pub fn gaussian_pdf(point1: Point, point2: Point, sigma: f32) -> f32 {
    use std::f32::consts::PI;
    // Calculate the Euclidean distance between point1 and point2
    let distance = point1.distance(point2);
    // Calculate the Gaussian probability density function
    let coefficient = 1.0 / (sigma * (2.0 * PI).sqrt());
    let exponent = - (distance.powi(2)) / (2.0 * sigma.powi(2));
    let result = coefficient * exponent.exp();
    //info!("distance: {}, result: {}", distance, result);
    // Normalize the result to be in the range 0-1.0
    let max_value = 1.0 / (sigma * (2.0 * PI).sqrt());
    let normalized_result = result / max_value;
    info!("distance: {}, result: {}", distance, normalized_result);
    normalized_result
}


/// Computes the exponential falloff between two points.
pub fn exponential_falloff(point1: Point, point2: Point) -> f32 {
    use std::f32::consts::E;
    let falloff_rate = 0.1;
    // Calculate the Euclidean distance between point1 and point2
    let distance = point1.distance(point2);
    // Calculate the exponential falloff
    E.powf(-falloff_rate * distance)
}