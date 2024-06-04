use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wolf_hash_map::WolfHashSet;
use wolf_serialise::WolfSerialise;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Angle(f64);

impl Angle {
    pub fn pi() -> Angle {
        Angle(PI)
    }
    pub fn zero() -> Angle {
        Angle(0.0)
    }
    pub fn cos(self) -> f64 {
        self.0.cos()
    }
    pub fn sin(self) -> f64 {
        self.0.sin()
    }
    pub fn enforce_range(raw: f64) -> Self {
        if raw >= 0.0 && raw < 2.0 * PI {
            return Angle(raw);
        }
        let divisor = (raw / (2.0 * PI)).floor();
        Angle(raw - divisor * 2.0 * PI)
    }
    pub fn assume_in_range(raw: f64) -> Self {
        Angle(raw)
    }
    pub fn degrees(&self) -> f64 {
        let full_turns = self.0 / (2.0 * PI);
        full_turns * 360.0
    }
    pub fn rotate_towards(self, other: Angle, speed: f64) -> Angle {
        let difference = Angle::enforce_range(other.0 - self.0);
        let reverse = Angle::assume_in_range(2.0 * PI - difference.0);
        let smallest = difference.0.min(reverse.0);
        if smallest <= speed {
            other
        } else {
            let rotation = if difference.0 < reverse.0 {
                speed
            } else {
                -speed
            };
            Angle::enforce_range(self.0 + rotation)
        }
    }
    pub fn write_byte_precision<W: std::io::Write>(
        &self,
        out_stream: &mut W,
    ) -> std::io::Result<()> {
        let rotations = self.0 / (2.0 * PI);
        let mut segments = (rotations * 256.0).round();
        if segments == 256.0 {
            segments = 0.0;
        }
        (segments as u8).wolf_serialise(out_stream)
    }
    pub fn read_byte_precision<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let segments = u8::wolf_deserialise(in_stream)?;
        let angle = (segments as f64 / 256.0) * 2.0 * PI;
        Ok(Angle::enforce_range(angle))
    }
}
impl Into<f64> for Angle {
    fn into(self) -> f64 {
        self.0
    }
}
impl Add for Angle {
    type Output = Angle;
    fn add(self, rhs: Angle) -> Angle {
        Angle::enforce_range(self.0 + rhs.0)
    }
}
impl Sub for Angle {
    type Output = Angle;
    fn sub(self, rhs: Angle) -> Angle {
        Angle::enforce_range(self.0 - rhs.0)
    }
}
impl Mul<f64> for Angle {
    type Output = Angle;
    fn mul(self, rhs: f64) -> Angle {
        Angle::enforce_range(self.0 * rhs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, WolfSerialise, Hash, EnumIter)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    pub fn closest_to_angle(angle: Angle) -> Self {
        let as_f64: f64 = angle.into();
        if as_f64 < PI / 4.0 {
            CardinalDirection::Right
        } else if as_f64 < 3.0 * PI / 4.0 {
            CardinalDirection::Up
        } else if as_f64 < 5.0 * PI / 4.0 {
            CardinalDirection::Left
        } else if as_f64 < 7.0 * PI / 4.0 {
            CardinalDirection::Down
        } else {
            CardinalDirection::Right
        }
    }
    /**
    returns every cardinal direction such that the angle difference
    between the angle and the cardinal is less than ninety degrees
    */
    pub fn compatible_with_angle(angle: Angle) -> WolfHashSet<CardinalDirection> {
        let mut ret = WolfHashSet::new();
        for direction in CardinalDirection::iter() {
            let diff = angle.sub(direction.into());
            if diff.0 < PI / 2.0 {
                ret.insert(direction);
            }
        }
        ret
    }
}

impl Into<Angle> for CardinalDirection {
    fn into(self) -> Angle {
        match self {
            CardinalDirection::Right => Angle::assume_in_range(0.0),
            CardinalDirection::Up => Angle::assume_in_range(PI / 2.0),
            CardinalDirection::Left => Angle::assume_in_range(PI),
            CardinalDirection::Down => Angle::assume_in_range(3.0 * PI / 2.0),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::f64::consts::PI;
    #[test]
    fn enforce_range_works() {
        let true_angle = 0.547;
        let raw_angle = 6.0 * PI + true_angle;
        let a = Angle::assume_in_range(true_angle);
        let b = Angle::enforce_range(raw_angle);
        assert!((a.0 - b.0).abs() < 0.0000001);
    }
    #[test]
    fn negative_enforce_range_works() {
        let true_angle = 2.0 * PI - 0.547;
        let raw_angle = -12.0 * PI + true_angle;
        let a = Angle::assume_in_range(true_angle);
        let b = Angle::enforce_range(raw_angle);
        assert!((a.0 - b.0).abs() < 0.0000001);
    }
    #[test]
    fn rotate_towards_close() {
        let target_angle = Angle::assume_in_range(PI * 1.5);
        let start_angle = Angle::assume_in_range(PI * 1.3);
        let rotated = start_angle.rotate_towards(target_angle, PI * 0.5);
        assert!((target_angle.0 - rotated.0).abs() < 0.0000001);
    }
    #[test]
    fn rotate_towards_forward() {
        let target_angle = Angle::assume_in_range(PI * 1.5);
        let start_angle = Angle::assume_in_range(PI * 1.0);
        let rotated = start_angle.rotate_towards(target_angle, PI * 0.3);
        assert!((rotated.0 - PI * 1.3).abs() < 0.0000001);
    }
    #[test]
    fn rotate_towards_backward() {
        let target_angle = Angle::assume_in_range(PI * 1.0);
        let start_angle = Angle::assume_in_range(PI * 1.5);
        let rotated = start_angle.rotate_towards(target_angle, PI * 0.3);
        assert!((rotated.0 - PI * 1.2).abs() < 0.0000001);
    }
}
