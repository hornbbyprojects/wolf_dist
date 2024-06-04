use crate::angle::Angle;
use crate::{Coords, Plane};
pub use fixed;
use fixed::traits::ToFixed;
use fixed::types::extra::U32;
use fixed::*;
use num_traits::{One, Zero};
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};
type PixelNumType = FixedI64<U32>;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, WolfSerialise)]
pub struct PixelNum(pub PixelNumType);

impl PixelNum {
    pub fn from_num<Src: ToFixed>(src: Src) -> Self {
        PixelNum(PixelNumType::from_num(src))
    }
    pub const fn const_from_int(src: i64) -> Self {
        PixelNum(PixelNumType::const_from_int(src))
    }
    pub fn to_num<To: fixed::traits::FromFixed>(self) -> To {
        self.0.to_num()
    }
}

//needs to use from_bits to be const (from_bits is a const fn)
pub const SQUARE_SIZE: PixelNum = PixelNum(FixedI64::<U32>::from_bits(40 << 32));

impl Neg for PixelNum {
    type Output = Self;
    fn neg(self) -> Self {
        PixelNum(self.0.neg())
    }
}

impl Div<Self> for PixelNum {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        PixelNum(self.0.div(other.0))
    }
}
impl Add<Self> for PixelNum {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        PixelNum(self.0.add(other.0))
    }
}
impl Sub<Self> for PixelNum {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        PixelNum(self.0.sub(other.0))
    }
}
impl Mul<f64> for PixelNum {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        PixelNum(self.0.mul(other.to_fixed::<PixelNumType>()))
    }
}
impl Mul<Self> for PixelNum {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        PixelNum(self.0.mul(other.0))
    }
}

impl Zero for PixelNum {
    fn zero() -> Self {
        PixelNum(0.into())
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl One for PixelNum {
    fn one() -> Self {
        PixelNum(1.into())
    }
    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, WolfSerialise)]
pub struct PixelCoords(pub Coords<PixelNum>);

impl PixelCoords {
    pub fn get_plane(&self) -> Plane {
        self.0.get_plane()
    }
    pub fn set_plane(&self, plane: Plane) -> Self {
        PixelCoords(self.0.set_plane(plane))
    }
    pub fn new_to_fixed<T: ToFixed>(plane: Plane, x: T, y: T) -> Self {
        Self::new(plane, PixelNum::from_num(x), PixelNum::from_num(y))
    }
    pub fn new(plane: Plane, x: PixelNum, y: PixelNum) -> Self {
        PixelCoords(Coords::new(plane, x, y))
    }
    pub fn new_at_zero() -> Self {
        Self::new(Plane(0), PixelNum::from_num(0), PixelNum::from_num(0))
    }
    pub fn translate_fixed(&self, dx: PixelNum, dy: PixelNum) -> Self {
        PixelCoords(self.0.translate(dx, dy))
    }
    pub fn translate<T: ToFixed>(&self, dx: T, dy: T) -> Self {
        let dx_fixed = PixelNum::from_num(dx);
        let dy_fixed = PixelNum::from_num(dy);
        self.translate_fixed(dx_fixed, dy_fixed)
    }
    pub fn stretch<T: ToFixed>(self, mx: T, my: T) -> Self {
        let fixed_x = PixelNum::from_num(mx);
        let fixed_y = PixelNum::from_num(my);
        let x = self.get_x() * fixed_x;
        let y = self.get_y() * fixed_y;
        PixelCoords::new(self.get_plane(), x, y)
    }
    pub fn difference(&self, other: &Self) -> Self {
        PixelCoords(self.0 - other.0)
    }
    pub fn get_y(&self) -> PixelNum {
        self.0.get_y()
    }
    pub fn get_x(&self) -> PixelNum {
        self.0.get_x()
    }
    pub fn get_direction_to(&self, other: &Self) -> Angle {
        let offset = *other - *self;
        let dx: f64 = offset.get_x().0.to_num();
        let dy: f64 = offset.get_y().0.to_num();
        let raw_angle = if dx == 0.0 {
            if dy >= 0.0 {
                PI * 0.5
            } else {
                PI * 1.5
            }
        } else {
            let small_angle = (dy / dx).atan();
            if dx < 0.0 {
                small_angle + PI
            } else {
                small_angle
            }
        };
        Angle::enforce_range(raw_angle)
    }
    pub fn get_distance_to(&self, other: &Self) -> f64 {
        let offset = *other - *self;
        let dx: f64 = offset.get_x().0.to_num();
        let dy: f64 = offset.get_y().0.to_num();

        (dx * dx + dy * dy).sqrt()
    }
    pub fn offset_direction(&self, direction: Angle, distance: f64) -> Self {
        let dx = direction.cos() * distance;
        let dy = direction.sin() * distance;
        self.translate(dx, dy)
    }
}
impl Sub<PixelCoords> for PixelCoords {
    type Output = Self;
    fn sub(self, other: PixelCoords) -> Self {
        PixelCoords(self.0.sub(other.0))
    }
}
impl Add<PixelCoords> for PixelCoords {
    type Output = Self;
    fn add(self, other: PixelCoords) -> Self {
        PixelCoords(self.0.add(other.0))
    }
}
impl<T: ToFixed> Mul<T> for PixelCoords {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        self * PixelNum::from_num(other)
    }
}
impl Mul<PixelNum> for PixelCoords {
    type Output = Self;
    fn mul(self, other: PixelNum) -> Self {
        PixelCoords(self.0.mul(other))
    }
}

impl Div<PixelNum> for PixelCoords {
    type Output = Self;
    fn div(self, other: PixelNum) -> Self {
        PixelCoords(self.0.div(other))
    }
}

impl<T: ToFixed> Div<T> for PixelCoords {
    type Output = Self;
    fn div(self, other: T) -> Self {
        self / PixelNum::from_num(other)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn positive_pixel_to_square_works() {
        let coords: SquareCoords = PixelCoords::new(
            Plane(0),
            SQUARE_SIZE * PixelNum::from_num(3.0),
            SQUARE_SIZE * PixelNum::from_num(2.5),
        )
        .into();
        assert!(coords.0.get_x() == 3);
        assert!(coords.0.get_y() == 2);
    }
    #[test]
    fn negative_pixel_to_square_works() {
        let coords: SquareCoords = PixelCoords::new(
            Plane(1),
            -SQUARE_SIZE * PixelNum::from_num(3.0),
            -SQUARE_SIZE * PixelNum::from_num(2.5),
        )
        .into();
        assert!(coords.0.get_plane() == Plane(1));
        assert!(coords.0.get_x() == -3);
        assert!(coords.0.get_y() == -3);
    }
    #[test]
    fn pixel_translate_works() {
        let start = PixelCoords::new_at_zero().translate(45.6, 45.6);
        assert_eq!(start.0.get_x(), PixelNum::from_num(45.6));
        assert_eq!(start.0.get_y(), PixelNum::from_num(45.6));
        let finish = start.translate(45.4, 0.0);
        assert_eq!(finish.0.get_x(), PixelNum::from_num(91));
        assert_eq!(finish.0.get_y(), PixelNum::from_num(45.6));
    }
}
