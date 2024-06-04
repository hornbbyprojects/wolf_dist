use crate::drawing::DrawingState;
use crate::sprites::*;
use coords::*;
use sdl2;
use sdl2::render;
use sdl2::render::{Texture, TextureAccess};
use std::collections::HashMap;
use wolf_interface::*;

pub const CHUNK_BUFFER: i64 = 120; // Extra pixels around the chunk, to allow for e.g. trees going off the top
pub const TERRAIN_CHUNK_SIZE_WITH_BUFFER: i64 = TERRAIN_CHUNK_SIZE_PIXELS + CHUNK_BUFFER * 2;

pub struct Terrain<'a> {
    chunks: HashMap<TerrainChunkCoords, Chunk<'a>>,
}
impl<'a> Terrain<'a> {
    pub fn new() -> Self {
        Terrain {
            chunks: HashMap::new(),
        }
    }
    pub fn add_chunk<T: render::RenderTarget>(
        &mut self,
        canvas: &mut render::Canvas<T>,
        texture_creator: &'a render::TextureCreator<sdl2::video::WindowContext>,
        sprites: &Sprites,
        message: ChunkInfoMessage,
    ) {
        let coords = message.coords;
        let chunk =
            Chunk::create_from_chunk_info_message(sprites, message.base, canvas, texture_creator);
        self.chunks.insert(coords, chunk);
    }
    pub fn update_chunk<T: render::RenderTarget>(
        &mut self,
        canvas: &mut render::Canvas<T>,
        sprites: &Sprites,
        message: ChunkUpdateMessage,
    ) {
        let chunk = self
            .chunks
            .get_mut(&message.coords)
            .expect("Tried to update missing chunk!");
        chunk.update(sprites, canvas, message);
    }
    pub fn unload_chunk(&mut self, message: ChunkUnloadMessage) {
        for coords in message.coords {
            self.chunks.remove(&coords);
        }
    }
    pub fn draw<T: render::RenderTarget>(
        &self,
        viewport_width: f64,
        viewport_height: f64,
        canvas: &mut render::Canvas<T>,
        _sprites: &Sprites,
        drawing_state: &DrawingState,
    ) {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let mut collected_coords: Vec<TerrainChunkCoords> =
            self.chunks.keys().map(|x| *x).collect();
        //higher up drawn first
        collected_coords.sort_by(|a, b| b.0.get_y().cmp(&a.0.get_y()));

        for coords in collected_coords {
            if coords.get_plane() != drawing_state.view_coords.get_plane() {
                // Dreams of another world...
                continue;
            }
            let chunk = self.chunks.get(&coords).unwrap();
            let pixel_coords: PixelCoords = coords.top_left_pixel();
            let dest_coords = pixel_coords.difference(&drawing_state.view_coords);
            let dest_x = dest_coords.0.get_x().0.to_num::<f64>() + viewport_width / 2.0
                - CHUNK_BUFFER as f64;
            //convert from server-side y-up to SDL y-down
            let dest_y = -dest_coords.0.get_y().0.to_num::<f64>() + viewport_height / 2.0
                - CHUNK_BUFFER as f64;

            let dest_rect = sdl2::rect::Rect::new(
                dest_x as i32,
                dest_y as i32,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
            );
            let source_rect = sdl2::rect::Rect::new(
                0,
                0,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
            );

            canvas
                .copy(&chunk.chunk_image, source_rect, dest_rect)
                .expect("Unable to draw chunk image to canvas!");
        }
    }
}

pub struct Chunk<'a> {
    pub chunk_image: Texture<'a>,
    pub base_sprite: u32,
}

impl<'a> Chunk<'a> {
    pub fn update<T: render::RenderTarget>(
        &mut self,
        sprites: &Sprites,
        canvas: &mut render::Canvas<T>,
        message: ChunkUpdateMessage,
    ) {
        let base_sprite = self.base_sprite;
        canvas
            .with_texture_canvas(&mut self.chunk_image, |canvas| {
                for (relative, new_sprites) in message.square_updates {
                    canvas.set_blend_mode(sdl2::render::BlendMode::None);
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 0));
                    let top_left_x = relative.0.get_x() as i64 * SQUARE_SIZE_PIXELS + CHUNK_BUFFER;
                    let top_left_y = TERRAIN_CHUNK_SIZE_PIXELS
                        - SQUARE_SIZE_PIXELS
                        - relative.0.get_y() as i64 * SQUARE_SIZE_PIXELS
                        + CHUNK_BUFFER;
                    canvas
                        .fill_rect(sdl2::rect::Rect::new(
                            top_left_x as i32,
                            top_left_y as i32,
                            SQUARE_SIZE_PIXELS as u32,
                            SQUARE_SIZE_PIXELS as u32,
                        ))
                        .unwrap();
                    Self::draw_square(
                        sprites,
                        canvas,
                        relative.0.get_x() as usize,
                        relative.0.get_y() as usize,
                        base_sprite,
                        new_sprites,
                    );
                }
            })
            .expect("Unable to draw to chunk image to update square");
    }
    fn draw_square<T: render::RenderTarget>(
        sprites: &Sprites,
        canvas: &mut render::Canvas<T>,
        dest_x_squares: usize,
        dest_y_squares: usize,
        base_sprite: u32,
        square_sprites: Vec<u32>,
    ) {
        let square_center_x = dest_x_squares as i32 * SQUARE_SIZE_PIXELS as i32
            + (SQUARE_SIZE_PIXELS / 2) as i32
            + CHUNK_BUFFER as i32;

        //more y is higher, so the square positioned at 0,0 is drawn at TERRAIN_CHUNK_SIZE_PIXELS - SQUARE_SIZE_PIXELS
        let square_center_y = TERRAIN_CHUNK_SIZE_PIXELS as i32
            - (dest_y_squares as i32 + 1) * SQUARE_SIZE_PIXELS as i32
            + (SQUARE_SIZE_PIXELS / 2) as i32
            + CHUNK_BUFFER as i32;

        for sprite in Some(base_sprite).into_iter().chain(square_sprites) {
            let sprite_info = sprites
                .terrain_sprite_infos
                .get(sprite)
                .expect(&format!("No sprite info for {}", sprite));

            let dest_x = square_center_x - sprite_info.center_x as i32;
            let dest_y = square_center_y - sprite_info.center_y as i32;

            let dest_rect = sdl2::rect::Rect::new(
                dest_x,
                dest_y,
                sprite_info.sprite_width,
                sprite_info.sprite_height,
            );
            canvas
                .copy_ex(
                    &sprites.terrain_sprite_sheet,
                    sprite_info.get_source_rect(),
                    dest_rect,
                    0.0,
                    None,  //no rotation, so no need to set center
                    false, //no flipping
                    false, //"
                )
                .expect("Unable to render chunk from terrain sprite sheet!");
        }
    }
    pub fn create_from_chunk_info_message<T: sdl2::render::RenderTarget>(
        sprites: &Sprites,
        message: BaseChunkMessage,
        canvas: &mut render::Canvas<T>,
        texture_creator: &'a render::TextureCreator<sdl2::video::WindowContext>,
    ) -> Self {
        let mut chunk_image = texture_creator
            .create_texture(
                Some(sdl2::pixels::PixelFormatEnum::ARGB8888),
                TextureAccess::Target,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
                TERRAIN_CHUNK_SIZE_WITH_BUFFER as u32,
            )
            .expect("Could not create chunk texture");
        let terrain = message.terrain;
        let base_sprite = message.base_sprite;
        chunk_image.set_blend_mode(sdl2::render::BlendMode::Blend);
        canvas
            .with_texture_canvas(&mut chunk_image, |canvas| {
                canvas.set_draw_color(sdl2::pixels::Color::RGBA(255, 255, 255, 0));
                canvas.set_blend_mode(sdl2::render::BlendMode::None);
                canvas.clear();
                for (i, square_sprites) in terrain.into_iter().enumerate() {
                    let dest_x_squares = i % TERRAIN_CHUNK_SIZE_SQUARES as usize;
                    let dest_y_squares = i / TERRAIN_CHUNK_SIZE_SQUARES as usize;
                    Self::draw_square(
                        sprites,
                        canvas,
                        dest_x_squares,
                        dest_y_squares,
                        base_sprite,
                        square_sprites,
                    );
                }
            })
            .expect("Failed to render to chunk image!");
        Chunk {
            chunk_image,
            base_sprite,
        }
    }
}
