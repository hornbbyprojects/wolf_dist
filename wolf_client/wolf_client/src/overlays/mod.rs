use coords::PixelCoords;
use sdl2::{
    rect::Rect,
    render::{self, Canvas, TextureCreator},
    video::WindowContext,
};
use sprite_mappings::NUN_SPRITE;
use wolf_interface::{AbilityCommand, SlotMappingMessage};

use crate::{client_side_component::Drawable, sprite_info::SpriteInfos, sprites::Sprites};

pub struct AbilitiesOverlay {
    last_mapping: Option<SlotMappingMessage>,
}
pub const TOP_LEFT_X: i32 = 10;
pub const TOP_LEFT_Y: i32 = 10;
impl AbilitiesOverlay {
    pub fn new() -> Self {
        AbilitiesOverlay { last_mapping: None }
    }
    pub fn update_slot_mapping(&mut self, message: SlotMappingMessage) {
        self.last_mapping = Some(message);
    }
    pub fn draw<T: render::RenderTarget>(&self, canvas: &mut Canvas<T>, sprites: &Sprites) {
        let sprite_info = SpriteInfos::ability_overlay_sprite_info();
        let source_rect = sprite_info.get_source_rect();
        let dest_rect = Rect::new(10, 10, sprite_info.sprite_width, sprite_info.sprite_height);
        canvas
            .copy(&sprites.ability_overlay, source_rect, dest_rect)
            .unwrap();
        if let Some(ref last_mapping) = self.last_mapping {
            for (i, icon) in last_mapping.slot_to_ability_icon.iter().enumerate() {
                if let Some(icon) = icon {
                    let x = (51 * i + 25) as i32 + TOP_LEFT_X;
                    let y = 25i32 + TOP_LEFT_Y;
                    let sprite_info = sprites.entity_sprite_infos.get(*icon).unwrap();
                    let source_rect = sprite_info.get_source_rect();
                    let x_centered = x - (sprite_info.sprite_width / 2) as i32;
                    let y_centered = y - (sprite_info.sprite_height / 2) as i32;
                    let dest_rect = Rect::new(
                        x_centered,
                        y_centered,
                        sprite_info.sprite_width as u32,
                        sprite_info.sprite_height as u32,
                    );
                    canvas
                        .copy(&sprites.main_sprite_sheet, source_rect, dest_rect)
                        .unwrap();
                }
            }
        }
    }
}
