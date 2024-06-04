use crate::sprite_info::*;
use sdl2::image::LoadSurface;
use sdl2::render;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct Sprites<'a> {
    pub main_sprite_sheet: render::Texture<'a>,
    pub terrain_sprite_sheet: render::Texture<'a>,
    pub animation_sprite_sheet: render::Texture<'a>,
    pub ability_overlay: render::Texture<'a>,

    pub entity_sprite_infos: SpriteInfos,
    pub terrain_sprite_infos: SpriteInfos,

    pub basic_font: Font<'a, 'a>,
    pub outline_font: Font<'a, 'a>,
}

impl<'a> Sprites<'a> {
    pub fn load(
        texture_creator: &'a render::TextureCreator<sdl2::video::WindowContext>,
        ttf: &'a mut Sdl2TtfContext,
    ) -> Self {
        let main_sprite_sheet_surface =
            sdl2::surface::Surface::from_file("images/main_sprite_sheet.png")
                .expect("Could not load main sprite sheet!");
        let main_sprite_sheet = texture_creator
            .create_texture_from_surface(&main_sprite_sheet_surface)
            .expect("Could not convert main sprite sheet to texture!");

        let terrain_sprite_sheet_surface =
            sdl2::surface::Surface::from_file("images/terrain_sprite_sheet.png")
                .expect("Could not load terrain sprite sheet!");
        let terrain_sprite_sheet = texture_creator
            .create_texture_from_surface(&terrain_sprite_sheet_surface)
            .expect("Could not convert terrain sprite sheet to texture!");

        let animation_sprite_sheet_surface =
            sdl2::surface::Surface::from_file("images/animation_sprite_sheet.png")
                .expect("Could not load terrain sprite sheet!");
        let animation_sprite_sheet = texture_creator
            .create_texture_from_surface(&animation_sprite_sheet_surface)
            .expect("Could not convert terrain sprite sheet to texture!");

        let ability_overlay_surface =
            sdl2::surface::Surface::from_file("images/ability_overlay.png")
                .expect("Could not load terrain sprite sheet!");
        let ability_overlay = texture_creator
            .create_texture_from_surface(&ability_overlay_surface)
            .expect("Could not convert terrain sprite sheet to texture!");

        let entity_sprite_infos = SpriteInfos::load_entity_sprite_infos();
        let terrain_sprite_infos = SpriteInfos::load_terrain_sprite_infos();

        let basic_font = ttf
            .load_font("fonts/basic.ttf", 12)
            .expect("Unable to load basic font!");
        let mut outline_font = ttf
            .load_font("fonts/basic.ttf", 12)
            .expect("Unable to load basic font!");
        outline_font.set_outline_width(1);

        Sprites {
            main_sprite_sheet,
            terrain_sprite_sheet,
            animation_sprite_sheet,
            entity_sprite_infos,
            terrain_sprite_infos,
            ability_overlay,
            basic_font,
            outline_font,
        }
    }
}
