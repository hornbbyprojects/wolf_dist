use super::*;

// Each city block width contains 2 * this many reservations + 1 (center), and some extra space
const CITY_BLOCK_RESERVATION_WIDTH: i64 = 2;
pub const CITY_BLOCK_SIZE: i64 = RESERVATION_CHUNK_SIZE * (2 * CITY_BLOCK_RESERVATION_WIDTH + 3);
pub type CityBlockChunkCoords = ChunkCoords<CITY_BLOCK_SIZE>;

// The city grows in BLOCKS
// Inside each block we dish out reservations
pub struct CityBlock {
    spiraler: Spiraler<RESERVATION_CHUNK_SIZE>,
    center: ReservationChunkCoords,
}

impl CityBlock {
    pub fn new(center: ReservationChunkCoords) -> Self {
        CityBlock {
            spiraler: Spiraler::new(),
            center,
        }
    }
    pub fn get_reservation(&mut self) -> ReservationChunkCoords {
        self.spiraler.get_next_coords() + self.center
    }
    pub fn is_full(&self) -> bool {
        self.spiraler.next_coords.get_x().abs() > CITY_BLOCK_RESERVATION_WIDTH
            || self.spiraler.next_coords.get_y().abs() > CITY_BLOCK_RESERVATION_WIDTH
    }
}
