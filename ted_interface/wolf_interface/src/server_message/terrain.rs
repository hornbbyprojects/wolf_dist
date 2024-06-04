use coords::*;
use wolf_serialise::WolfSerialise;

#[derive(Debug, PartialEq, Clone)]
pub struct BaseChunkMessage {
    //what is drawn at the bottom of each square
    pub base_sprite: u32,
    //ordinary floor terrain
    pub terrain: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq, Clone, WolfSerialise)]
pub struct ChunkUpdateMessage {
    pub coords: TerrainChunkCoords,
    pub square_updates: Vec<(ChunkRelativeSquareCoords, Vec<u32>)>,
}

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct ChunkInfoMessage {
    pub coords: TerrainChunkCoords,
    pub base: BaseChunkMessage,
}
#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct ChunkUnloadMessage {
    pub coords: Vec<TerrainChunkCoords>,
}

type SquareLen = i16;

impl WolfSerialise for BaseChunkMessage {
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        let mut empty_counter: SquareLen = 0;
        self.base_sprite.wolf_serialise(out_stream)?;
        for square in self.terrain.iter() {
            if square.is_empty() {
                empty_counter += 1;
                continue;
            }
            if empty_counter > 0 {
                (-empty_counter).wolf_serialise(out_stream)?;
                empty_counter = 0;
            }
            (square.len() as SquareLen).wolf_serialise(out_stream)?;
            for sprite in square.iter() {
                sprite.wolf_serialise(out_stream)?;
            }
        }
        if empty_counter > 0 {
            (-empty_counter).wolf_serialise(out_stream)?;
        }
        Ok(())
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let base_sprite = u32::wolf_deserialise(in_stream)?;
        let mut terrain = Vec::new();
        let mut squares_counted: SquareLen = 0;
        while squares_counted < TERRAIN_CHUNK_AREA_SQUARES as SquareLen {
            let length = SquareLen::wolf_deserialise(in_stream)?;
            if length < 0 {
                for _ in 0..-length {
                    terrain.push(vec![]);
                }
                squares_counted -= length;
                continue;
            }
            let mut square = Vec::new();
            for _ in 0..length {
                let sprite = u32::wolf_deserialise(in_stream)?;
                square.push(sprite);
            }
            terrain.push(square);
            squares_counted += 1;
        }
        Ok(BaseChunkMessage {
            base_sprite,
            terrain,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::server_message::terrain::*;
    use std::io::Cursor;
    fn test_chunk_compare(a: &BaseChunkMessage, b: &BaseChunkMessage) {
        if a.terrain.len() != TERRAIN_CHUNK_AREA_SQUARES as usize {
            panic!("Wrong terrain length {} for BCM a!", a.terrain.len())
        }
        if b.terrain.len() != TERRAIN_CHUNK_AREA_SQUARES as usize {
            panic!("Wrong terrain length {} for BCM b!", b.terrain.len())
        }
        for i in 0..TERRAIN_CHUNK_AREA_SQUARES as usize {
            let square_a = &a.terrain[i];
            let square_b = &b.terrain[i];
            if square_a.len() != square_b.len() {
                panic!(
                    "Square lengths at {} ({}, {}) vary!",
                    i,
                    square_a.len(),
                    square_b.len()
                );
            }
            for j in 0..square_a.len() {
                let sprite_a = square_a[j];
                let sprite_b = square_b[j];
                if sprite_a != sprite_b {
                    panic!(
                        "Square lengths at {}:{} ({}, {}) vary!A",
                        i, j, sprite_a, sprite_b
                    );
                }
            }
        }
    }
    #[test]
    fn empty_chunk_works() {
        let mut terrain = Vec::new();
        for _ in 0..TERRAIN_CHUNK_AREA_SQUARES {
            terrain.push(vec![]);
        }
        let base_chunk_message = BaseChunkMessage {
            base_sprite: 0,
            terrain,
        };
        let mut cursor = Cursor::new(vec![]);
        base_chunk_message
            .wolf_serialise(&mut cursor)
            .expect("Could not serialise chunk!");
        let mut read_cursor = Cursor::new(cursor.into_inner());
        let new_base_chunk_message = BaseChunkMessage::wolf_deserialise(&mut read_cursor)
            .expect("Could not deserialise chunk!");
        test_chunk_compare(&base_chunk_message, &new_base_chunk_message);
        let next_byte = u8::wolf_deserialise(&mut read_cursor);
        assert!(next_byte.is_err())
    }
    #[test]
    fn filled_chunk_works() {
        let mut terrain = Vec::new();
        for i in 0..TERRAIN_CHUNK_AREA_SQUARES {
            if i % 7 == 2 {
                terrain.push(vec![2, 3, 1]);
            } else {
                terrain.push(vec![]);
            }
        }
        let base_chunk_message = BaseChunkMessage {
            base_sprite: 0,
            terrain,
        };
        let mut cursor = Cursor::new(vec![]);
        base_chunk_message
            .wolf_serialise(&mut cursor)
            .expect("Could not serialise chunk!");
        let mut read_cursor = Cursor::new(cursor.into_inner());
        let new_base_chunk_message = BaseChunkMessage::wolf_deserialise(&mut read_cursor)
            .expect("Could not deserialise chunk!");
        test_chunk_compare(&base_chunk_message, &new_base_chunk_message);
        let next_byte = u8::wolf_deserialise(&mut read_cursor);
        assert!(next_byte.is_err())
    }
}
