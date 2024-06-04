use coords::{Angle, PixelCoords, Plane};

const ROUND_SMALL_AMOUNT: f64 = 1024.0; //power of 2 chosen so result is always representable

pub fn round_small(input: f64) -> f64 {
    (input * ROUND_SMALL_AMOUNT).round() / ROUND_SMALL_AMOUNT
}

pub fn vector_magnitude(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

pub fn vector_angle(x: f64, y: f64) -> Angle {
    PixelCoords::new_at_zero().get_direction_to(&PixelCoords::new_to_fixed(Plane(0), x, y))
}
