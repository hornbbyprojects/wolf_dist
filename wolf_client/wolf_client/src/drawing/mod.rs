use crate::overlays::AbilitiesOverlay;
use crate::sprites::*;
use coords::*;
use sdl2;
use sdl2::render;
use sdl2::ttf::Font;
use wolf_interface::*;

mod terrain;
use terrain::*;
pub mod messages;
pub mod speech;

pub struct Drawing<'a> {
    pub last_state: Option<DrawingState>,
    pub next_state: DrawingState,
    pub terrain: Terrain<'a>,
    pub abilities_overlay: AbilitiesOverlay,
}

impl<'a> Drawing<'a> {
    pub fn new() -> Self {
        Drawing {
            last_state: None,
            next_state: DrawingState::new(),
            terrain: Terrain::new(),
            abilities_overlay: AbilitiesOverlay::new(),
        }
    }
    pub fn get_view_coords(&self) -> PixelCoords {
        self.next_state.view_coords
    }
    pub fn draw<T: render::RenderTarget>(
        &self,
        viewport_width: f64,
        viewport_height: f64,
        canvas: &mut render::Canvas<T>,
        texture_creator: &'a render::TextureCreator<sdl2::video::WindowContext>,
        sprites: &mut Sprites,
    ) {
        canvas.clear();

        self.terrain.draw(
            viewport_width,
            viewport_height,
            canvas,
            sprites,
            &self.next_state,
        );
    }
    pub fn update_view(&mut self, view_message: &ViewMessage) {
        self.next_state.current_frame = self.next_state.current_frame + 1;
        self.next_state.view_coords = view_message.view_coords;
    }
    pub fn add_chunk<T: render::RenderTarget>(
        &mut self,
        canvas: &mut render::Canvas<T>,
        texture_creator: &'a render::TextureCreator<sdl2::video::WindowContext>,
        sprites: &Sprites,
        message: ChunkInfoMessage,
    ) {
        self.terrain
            .add_chunk(canvas, texture_creator, sprites, message);
    }
    pub fn update_chunk<T: render::RenderTarget>(
        &mut self,
        canvas: &mut render::Canvas<T>,
        sprites: &Sprites,
        message: ChunkUpdateMessage,
    ) {
        self.terrain.update_chunk(canvas, sprites, message);
    }
    pub fn unload_chunk(&mut self, message: ChunkUnloadMessage) {
        self.terrain.unload_chunk(message);
    }
}

#[derive(Clone)]
pub struct DrawingState {
    pub view_coords: PixelCoords,
    pub current_frame: u32,
}

impl DrawingState {
    pub fn new() -> Self {
        DrawingState {
            view_coords: PixelCoords::new_at_zero(),
            current_frame: 0,
        }
    }
    pub fn interpolate(&self, other: &Self, percent_through_frame: f64) -> Self {
        DrawingState {
            view_coords: self.view_coords * (1.0 - percent_through_frame)
                + other.view_coords * percent_through_frame,
            current_frame: 0,
        }
    }
}
