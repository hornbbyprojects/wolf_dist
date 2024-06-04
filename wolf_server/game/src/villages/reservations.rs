use super::*;

pub const RESERVATION_CHUNK_SIZE: i64 = 9;
pub type ReservationChunkCoords = ChunkCoords<RESERVATION_CHUNK_SIZE>;
pub struct Spiraler<const CHUNK_SIZE: i64> {
    pub next_coords: ChunkCoords<CHUNK_SIZE>,
    pub direction: (i64, i64),
    pub segment_length: u8,
    pub segment_counter: u8,
    pub increase_segment_length_next: bool,
}
impl<const CHUNK_SIZE: i64> Spiraler<CHUNK_SIZE> {
    pub fn new() -> Self {
        Spiraler {
            next_coords: ChunkCoords::<CHUNK_SIZE>::new(Plane(0), 0, 0),
            direction: (1, 0),
            increase_segment_length_next: false,
            segment_length: 2,
            segment_counter: 0,
        }
    }
    fn rotate_direction(&mut self) {
        let dx = -self.direction.1;
        let dy = self.direction.0;
        self.direction = (dx, dy);
        self.segment_counter = 1;
        if self.increase_segment_length_next {
            self.increase_segment_length_next = false;
            self.segment_length += 1;
        } else {
            self.increase_segment_length_next = true;
        }
    }
    pub fn get_next_coords(&mut self) -> ChunkCoords<CHUNK_SIZE> {
        let next_coords = self.next_coords.clone();
        self.segment_counter += 1;
        if self.segment_counter >= self.segment_length {
            self.rotate_direction();
        }
        self.next_coords = self
            .next_coords
            .translate(self.direction.0, self.direction.1);
        next_coords
    }
}
