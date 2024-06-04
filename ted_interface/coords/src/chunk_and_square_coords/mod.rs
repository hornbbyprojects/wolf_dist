use super::pixel_coords::*;
use crate::{Coords, Plane};

mod rectangle;
pub use rectangle::*;
use wolf_serialise::WolfSerialise;

pub const TERRAIN_CHUNK_SIZE_SQUARES: i64 = 16;
pub const SQUARE_SIZE_PIXELS: i64 = 40;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, WolfSerialise)]
pub struct SquareCoords(pub Coords<i64>);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, WolfSerialise)]
pub struct ChunkRelativeSquareCoords(pub Coords<u8>);
#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct ChunkCoords<const CHUNK_SIZE: i64>(pub Coords<i64>);
pub type TerrainChunkCoords = ChunkCoords<TERRAIN_CHUNK_SIZE_SQUARES>;

// TODO: Make derive wolf serialise work on const generics
impl<const CHUNK_SIZE: i64> WolfSerialise for ChunkCoords<CHUNK_SIZE> {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        self.0.wolf_serialise(out_stream)?;
        Ok(())
    }

    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        Ok(ChunkCoords(Coords::<i64>::wolf_deserialise(in_stream)?))
    }
}
const fn get_pixel_chunk_size(chunk_size: i64) -> i64 {
    chunk_size * SQUARE_SIZE_PIXELS
}

pub const fn get_chunk_area(chunk_size: i64) -> i64 {
    chunk_size * chunk_size
}

pub const TERRAIN_CHUNK_AREA_SQUARES: i64 = get_chunk_area(TERRAIN_CHUNK_SIZE_SQUARES);
pub const TERRAIN_CHUNK_SIZE_PIXELS: i64 = get_pixel_chunk_size(TERRAIN_CHUNK_SIZE_SQUARES);

impl From<Coords<i64>> for SquareCoords {
    fn from(coords: Coords<i64>) -> Self {
        SquareCoords(coords)
    }
}
impl<const CHUNK_SIZE: i64> From<Coords<i64>> for ChunkCoords<CHUNK_SIZE> {
    fn from(coords: Coords<i64>) -> Self {
        ChunkCoords(coords)
    }
}
impl From<Coords<u8>> for ChunkRelativeSquareCoords {
    fn from(coords: Coords<u8>) -> Self {
        ChunkRelativeSquareCoords(coords)
    }
}

impl From<SquareCoords> for Coords<i64> {
    fn from(coords: SquareCoords) -> Self {
        coords.0
    }
}
impl<const CHUNK_SIZE: i64> From<ChunkCoords<CHUNK_SIZE>> for Coords<i64> {
    fn from(coords: ChunkCoords<CHUNK_SIZE>) -> Self {
        coords.0
    }
}
impl From<ChunkRelativeSquareCoords> for Coords<u8> {
    fn from(coords: ChunkRelativeSquareCoords) -> Self {
        coords.0
    }
}

impl ChunkRelativeSquareCoords {
    pub fn new(x: u8, y: u8) -> Self {
        ChunkRelativeSquareCoords(Coords::<u8>::new(Plane(0), x, y))
    }
}
impl SquareCoords {
    pub fn translate(&self, x: i64, y: i64) -> Self {
        SquareCoords(self.0.translate(x, y))
    }
    pub fn new(plane: Plane, x: i64, y: i64) -> Self {
        SquareCoords(Coords::<i64>::new(plane, x, y))
    }
    pub fn get_x(&self) -> i64 {
        self.0.get_x()
    }
    pub fn get_y(&self) -> i64 {
        self.0.get_y()
    }
    pub fn get_plane(&self) -> Plane {
        self.0.get_plane()
    }
    pub fn bottom_left_pixel(&self) -> PixelCoords {
        PixelCoords::new(
            self.0.get_plane(),
            PixelNum::from_num(self.get_x() * SQUARE_SIZE_PIXELS),
            PixelNum::from_num(self.get_y() * SQUARE_SIZE_PIXELS),
        )
    }
    pub fn top_left_pixel(&self) -> PixelCoords {
        self.bottom_left_pixel().translate(0, SQUARE_SIZE_PIXELS)
    }
    pub fn center_pixel(&self) -> PixelCoords {
        self.bottom_left_pixel()
            .translate(SQUARE_SIZE_PIXELS / 2, SQUARE_SIZE_PIXELS / 2)
    }
    pub fn relative_to_chunk<const CHUNK_SIZE: i64>(
        self,
        chunk_coords: ChunkCoords<CHUNK_SIZE>,
    ) -> Option<ChunkRelativeSquareCoords> {
        let bottom_left = chunk_coords.bottom_left();
        let relative = self - bottom_left;
        let relative_x = relative.get_x();
        let relative_y = relative.get_y();
        if relative_x < 0 || relative_x >= CHUNK_SIZE || relative_y < 0 || relative_y >= CHUNK_SIZE
        {
            return None;
        }

        Some(ChunkRelativeSquareCoords::new(
            relative.get_x() as u8,
            relative.get_y() as u8,
        ))
    }
}
impl std::ops::Add for SquareCoords {
    type Output = SquareCoords;
    fn add(self, rhs: SquareCoords) -> SquareCoords {
        SquareCoords(self.0 + rhs.0)
    }
}
impl std::ops::Sub for SquareCoords {
    type Output = SquareCoords;
    fn sub(self, rhs: SquareCoords) -> SquareCoords {
        SquareCoords(self.0 - rhs.0)
    }
}
impl<const CHUNK_SIZE: i64> std::ops::Add<ChunkRelativeSquareCoords> for ChunkCoords<CHUNK_SIZE> {
    type Output = SquareCoords;
    fn add(self, rhs: ChunkRelativeSquareCoords) -> SquareCoords {
        let bottom_left = self.bottom_left();
        SquareCoords::new(
            self.get_plane(),
            rhs.0.get_x() as i64 + bottom_left.get_x(),
            rhs.0.get_y() as i64 + bottom_left.get_y(),
        )
    }
}
impl<const CHUNK_SIZE: i64> std::ops::Add for ChunkCoords<CHUNK_SIZE> {
    type Output = Self;
    fn add(self, rhs: ChunkCoords<CHUNK_SIZE>) -> Self {
        ChunkCoords(self.0 + rhs.0)
    }
}
impl From<PixelCoords> for SquareCoords {
    fn from(px: PixelCoords) -> Self {
        let square_x: i64 = (px.0.get_x() / SQUARE_SIZE).0.to_num();
        let square_y: i64 = (px.0.get_y() / SQUARE_SIZE).0.to_num();
        SquareCoords::new(px.get_plane(), square_x, square_y)
    }
}
impl<const CHUNK_SIZE: i64> From<PixelCoords> for ChunkCoords<CHUNK_SIZE> {
    fn from(px: PixelCoords) -> Self {
        let chunk_x: i64 = (px.0.get_x() / PixelNum::from_num(get_pixel_chunk_size(CHUNK_SIZE)))
            .0
            .to_num();
        let chunk_y: i64 = (px.0.get_y() / PixelNum::from_num(get_pixel_chunk_size(CHUNK_SIZE)))
            .0
            .to_num();
        ChunkCoords::new(px.get_plane(), chunk_x, chunk_y)
    }
}

impl<const CHUNK_SIZE: i64> ChunkCoords<CHUNK_SIZE> {
    pub fn get_x(&self) -> i64 {
        self.0.get_x()
    }
    pub fn get_y(&self) -> i64 {
        self.0.get_y()
    }
    pub fn get_plane(&self) -> Plane {
        self.0.get_plane()
    }
    pub fn translate(&self, x: i64, y: i64) -> Self {
        ChunkCoords(self.0.translate(x, y))
    }
    pub fn new(plane: Plane, x: i64, y: i64) -> Self {
        ChunkCoords(Coords::<i64>::new(plane, x, y))
    }
    pub fn top_left(self) -> SquareCoords {
        // Top left corner of chunk - useful for drawing
        self.bottom_left().translate(0, CHUNK_SIZE - 1)
    }
    pub fn top_left_pixel(self) -> PixelCoords {
        self.top_left().top_left_pixel()
    }
    pub fn bottom_left(self) -> SquareCoords {
        let cx = self.0.get_x();
        let cy = self.0.get_y();
        let sx = cx * CHUNK_SIZE;
        let sy = cy * CHUNK_SIZE;
        SquareCoords::new(self.get_plane(), sx, sy)
    }
    pub fn bottom_left_pixel(self) -> PixelCoords {
        let cx = self.0.get_x();
        let cy = self.0.get_y();
        let px = cx * get_pixel_chunk_size(CHUNK_SIZE);
        let py = cy * get_pixel_chunk_size(CHUNK_SIZE);
        PixelCoords::new_to_fixed(self.get_plane(), px, py)
    }
    pub fn top_right(self) -> SquareCoords {
        self.bottom_left().translate(CHUNK_SIZE - 1, CHUNK_SIZE - 1)
    }
    pub fn pixel_offset(&self, pixel_offset: PixelCoords) -> PixelCoords {
        let bottom_left_pixel = self.bottom_left_pixel();
        bottom_left_pixel + pixel_offset
    }
    pub fn center_square(self) -> SquareCoords {
        self.bottom_left().translate(CHUNK_SIZE / 2, CHUNK_SIZE / 2)
    }
    pub fn center_pixel(self) -> PixelCoords {
        self.bottom_left_pixel()
            + PixelCoords::new_to_fixed(
                self.get_plane(),
                get_pixel_chunk_size(CHUNK_SIZE) / 2,
                get_pixel_chunk_size(CHUNK_SIZE) / 2,
            )
    }
}
impl<const CHUNK_SIZE: i64> std::ops::Div<i64> for ChunkCoords<CHUNK_SIZE> {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        ChunkCoords(self.0 / rhs)
    }
}
fn square_coord_to_chunk_coord<const CHUNK_SIZE: i64>(coord: i64) -> i64 {
    if coord >= 0 {
        coord / CHUNK_SIZE
    } else {
        // We want to round towards negative inf, while rust rounds towards 0
        -((-coord - 1) / CHUNK_SIZE) - 1
    }
}
impl<const CHUNK_SIZE: i64> From<SquareCoords> for ChunkCoords<CHUNK_SIZE> {
    fn from(square_coords: SquareCoords) -> Self {
        let coords = square_coords.0.clone();
        let sx = coords.get_x();
        let sy = coords.get_y();
        let cx = square_coord_to_chunk_coord::<CHUNK_SIZE>(sx);
        let cy = square_coord_to_chunk_coord::<CHUNK_SIZE>(sy);
        ChunkCoords::new(square_coords.get_plane(), cx, cy)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    fn assert_square_conversion(sx: i64, sy: i64, cx: i64, cy: i64) {
        let cc: TerrainChunkCoords = SquareCoords::new(Plane(1), sx, sy).into();
        assert_eq!(cc, TerrainChunkCoords::new(Plane(1), cx, cy));
    }

    #[test]
    fn convert_square_to_chunk_works() {
        assert_square_conversion(0, 0, 0, 0);
        assert_square_conversion(-1, 0, -1, 0);
        assert_square_conversion(-1, -1, -1, -1);
        assert_square_conversion(1, 1, 0, 0);
        assert_square_conversion(TERRAIN_CHUNK_SIZE_SQUARES, 1, 1, 0);
        assert_square_conversion(1, TERRAIN_CHUNK_SIZE_SQUARES, 0, 1);
        assert_square_conversion(-TERRAIN_CHUNK_SIZE_SQUARES, 1, -1, 0);
        assert_square_conversion(1, -TERRAIN_CHUNK_SIZE_SQUARES, 0, -1);
    }
    fn chunk_relative_circle(sx: i64, sy: i64) {
        let coords = SquareCoords::new(Plane(0), sx, sy);
        let cc: TerrainChunkCoords = coords.into();
        let relative = coords.relative_to_chunk(cc).expect(&format!(
            "Mismatch in base chunk between into and relative_to_chunk with {:?} and {:?}",
            coords, cc
        ));
        assert_eq!(cc + relative, coords);
    }
    #[test]
    fn chunk_relative_coords_works() {
        chunk_relative_circle(0, 0);
        chunk_relative_circle(1, 0);
        chunk_relative_circle(0, 1);
        chunk_relative_circle(-1, 1);
        chunk_relative_circle(1, -1);
        chunk_relative_circle(0, -TERRAIN_CHUNK_SIZE_SQUARES);
    }
    #[test]
    fn square_of_coords_works() {
        const WIDTH: i64 = 13;
        const HEIGHT: i64 = 19;
        let bottom_left = SquareCoords::new(Plane(0), 0, 0);
        let top_right = SquareCoords::new(Plane(0), WIDTH - 1, HEIGHT - 1);
        let square = square_of_coords::<i64, SquareCoords>(bottom_left.clone(), top_right.clone());
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let coords = SquareCoords::new(Plane(0), x, y);
                assert!(square.contains(&coords));
            }
        }
        let left_edge_coords = SquareCoords::new(Plane(0), -1, 0);
        let top_edge_coords = SquareCoords::new(Plane(0), 0, -1);
        let right_edge_coords = SquareCoords::new(Plane(0), 0, HEIGHT);
        let bottom_edge_coords = SquareCoords::new(Plane(0), WIDTH, 0);
        let corner_coords = SquareCoords::new(Plane(0), WIDTH, HEIGHT);
        assert!(!square.contains(&left_edge_coords));
        assert!(!square.contains(&top_edge_coords));
        assert!(!square.contains(&right_edge_coords));
        assert!(!square.contains(&bottom_edge_coords));
        assert!(!square.contains(&corner_coords));
    }
}
