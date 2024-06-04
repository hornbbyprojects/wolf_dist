use super::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct SquareRectangle {
    centre_coords: SquareCoords,
    width: i64,
    height: i64,
}

impl SquareRectangle {
    pub fn new(centre_coords: SquareCoords, width: i64, height: i64) -> SquareRectangle {
        SquareRectangle {
            width,
            height,
            centre_coords,
        }
    }
    pub fn get_random_square(&self) -> SquareCoords {
        let dx = rand::thread_rng().gen_range(-self.width..=self.width);
        let dy = rand::thread_rng().gen_range(-self.height..=self.height);
        self.centre_coords.translate(dx, dy)
    }
    pub fn translate(&self, dx: i64, dy: i64) -> Self {
        SquareRectangle {
            width: self.width,
            height: self.height,
            centre_coords: self.centre_coords.translate(dx, dy),
        }
    }
}
