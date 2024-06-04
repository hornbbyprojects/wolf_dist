#![feature(step_trait)]
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};
#[macro_use]
extern crate wolf_serialise_derive;

mod angle;
mod chunk_and_square_coords;
mod pixel_coords;

pub use angle::*;
pub use chunk_and_square_coords::*;
pub use pixel_coords::*;
use wolf_hash_map::WolfHashSet;

#[derive(Copy, Clone, WolfSerialise, Eq, PartialEq, Hash, Debug)]
pub struct Plane(pub u32);

impl From<u32> for Plane {
    fn from(x: u32) -> Self {
        Plane(x)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, WolfSerialise)]
pub struct Coords<T> {
    plane: Plane,
    coords: [T; 2],
}

impl<T: Copy> Coords<T> {
    pub fn new(plane: Plane, x: T, y: T) -> Self {
        Coords {
            plane,
            coords: [x, y],
        }
    }
    pub fn get_plane(&self) -> Plane {
        self.plane
    }
    pub fn get_x(&self) -> T {
        self.coords[0]
    }
    pub fn get_y(&self) -> T {
        self.coords[1]
    }
}
impl<T: Add<Output = T> + Copy> Coords<T> {
    pub fn translate(&self, dx: T, dy: T) -> Self {
        Coords {
            plane: self.plane,
            coords: [self.coords[0] + dx, self.coords[1] + dy],
        }
    }
    pub fn set_plane(&self, plane: Plane) -> Self {
        let mut ret = self.clone();
        ret.plane = plane;
        ret
    }
}
impl<T: Add<Output = T> + Copy> Add<Coords<T>> for Coords<T> {
    type Output = Self;
    fn add(self, other: Coords<T>) -> Self {
        if self.plane != other.plane {
            panic!("Attempted to add coords with mismatched planes");
        }
        Coords {
            plane: self.plane,
            coords: [
                self.coords[0] + other.coords[0],
                self.coords[1] + other.coords[1],
            ],
        }
    }
}
impl<T: Sub<Output = T> + Copy> Sub<Coords<T>> for Coords<T> {
    type Output = Self;
    fn sub(self, other: Coords<T>) -> Self {
        if self.plane != other.plane {
            panic!("Attempted to substract coords with mismatched planes");
        }
        Coords {
            plane: self.plane,
            coords: [
                self.coords[0] - other.coords[0],
                self.coords[1] - other.coords[1],
            ],
        }
    }
}

impl<U: Copy, T: Mul<U, Output = T> + Copy> Mul<U> for Coords<T> {
    type Output = Self;
    fn mul(self, scale: U) -> Self {
        Coords {
            plane: self.plane,
            coords: [self.coords[0] * scale, self.coords[1] * scale],
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Coords<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Coords::new(self.plane, self.coords[0] / other, self.coords[1] / other)
    }
}
pub fn square_of_coords<T, U>(bottom_left: U, top_right: U) -> WolfHashSet<U>
where
    T: Copy + std::iter::Step + num_traits::One + Add<Output = T>,
    U: Hash + Eq + From<Coords<T>>,
    Coords<T>: From<U>,
{
    let mut ret = WolfHashSet::new();
    let bottom_left_coords = Coords::<T>::from(bottom_left);
    let top_right_coords = Coords::<T>::from(top_right);
    for x in bottom_left_coords.get_x()..(top_right_coords.get_x() + T::one()) {
        for y in bottom_left_coords.get_y()..(top_right_coords.get_y() + T::one()) {
            ret.insert(Coords::new(bottom_left_coords.plane, x, y).into());
        }
    }
    ret
}

pub fn square_of_coords_centered<T: Add<Output = T>, U: From<Coords<T>>>(
    center: U,
    radius: T,
) -> WolfHashSet<U>
where
    T: Copy + std::iter::Step + num_traits::One + Add<Output = T> + std::ops::Neg<Output = T>,
    U: Hash + Eq + From<Coords<T>> + Clone,
    Coords<T>: From<U>,
{
    let bottom_left = Coords::<T>::from(center.clone()).translate(-radius, -radius);
    let top_right = Coords::<T>::from(center).translate(radius, radius);
    square_of_coords(bottom_left.into(), top_right.into())
}
