use std::f64::consts::PI;

use coords::{Angle, PixelCoords};

/// A fraction of a turn, used to send rough angles to save space
const ONE_FRACTION: f64 = 2.0 * PI / 256.0;

/// Describes an angle in fractions of a whole turn, where RoughAngle(0) = RoughAngle(255) + RoughAngle(1)
#[derive(Debug, Clone, PartialEq, Eq, WolfSerialise)]
pub struct RoughAngle(pub u8);

impl From<Angle> for RoughAngle {
    fn from(precise_angle: Angle) -> Self {
        let fractions: f64 = (Into::<f64>::into(precise_angle) / ONE_FRACTION).round();
        if fractions >= 0.0 && fractions <= 255.0 {
            RoughAngle(fractions as u8)
        } else if fractions == 256.0 {
            RoughAngle(0)
        } else {
            panic!(
                "Invalid angle {:?} to make rough (produces {:?} fractions)!",
                precise_angle, fractions
            );
        }
    }
}

impl Into<Angle> for RoughAngle {
    fn into(self) -> Angle {
        Angle::enforce_range(self.0 as f64 * ONE_FRACTION)
    }
}
// also handles creation
#[derive(Debug, Clone, PartialEq, Eq, WolfSerialise)]
pub struct MoveGameObject {
    pub game_object_id: u32,
    pub coords: PixelCoords,
    pub rotation: RoughAngle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, WolfSerialise)]
pub struct RemoveGameObject {
    pub game_object_id: u32,
}
