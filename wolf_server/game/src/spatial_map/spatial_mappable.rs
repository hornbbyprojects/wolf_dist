use crate::game::Game;
use coords::square_of_coords;
use coords::{SquareCoords, TerrainChunkCoords};
use wolf_hash_map::WolfHashSet;

pub trait SpatialMappable {
    fn get_overlapping_chunks(&self, game: &Game) -> WolfHashSet<TerrainChunkCoords>;
}
pub trait SpatialBox {
    fn get_coords(&self, game: &Game) -> SquareCoords;
    fn get_box_dimensions(&self, _game: &Game) -> (u32, u32) {
        (1, 1)
    }
}
impl<T: SpatialBox> SpatialMappable for T {
    fn get_overlapping_chunks(&self, game: &Game) -> WolfHashSet<TerrainChunkCoords> {
        let current_coords = self.get_coords(game);
        let (width, height) = self.get_box_dimensions(game);

        let top_left = current_coords.translate(-(width as i64), -(height as i64));
        let bottom_right = current_coords.translate(width as i64, height as i64);

        square_of_coords(top_left.into(), bottom_right.into())
    }
}
