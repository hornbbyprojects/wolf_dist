use crate::client_side_component::Game;
use coords::*;
use sdl2::rect::Rect;
use sprite_mappings::*;
use std::collections::HashMap;

pub struct SpriteInfos {
    pub id_to_sprite_info: HashMap<u32, SpriteInfo>,
}
const CREATURE_SCALING: f64 = 0.5;

impl SpriteInfos {
    pub fn load_terrain_sprite_infos() -> Self {
        let mut id_to_sprite_info = HashMap::new();
        id_to_sprite_info.insert(0, SpriteInfo::new_default_size(160, 0));
        id_to_sprite_info.insert(1, SpriteInfo::new_default_size(200, 0));
        id_to_sprite_info.insert(GRASS_SPRITE, SpriteInfo::new_default_size(320, 40));
        id_to_sprite_info.insert(VOID_SPRITE, SpriteInfo::new_default_size(200, 120));
        id_to_sprite_info.insert(WALL_SPRITE, SpriteInfo::new_default_size(360, 0));
        id_to_sprite_info.insert(DOOR_SPRITE, SpriteInfo::new(169, 124, 19, 31));
        id_to_sprite_info.insert(DIRT_SPRITE, SpriteInfo::new_default_size(160, 80));
        id_to_sprite_info.insert(SCAFFOLD_SPRITE, SpriteInfo::new_default_size(240, 120));
        id_to_sprite_info.insert(FLOOR_SPRITE, SpriteInfo::new_default_size(120, 120));
        id_to_sprite_info.insert(WATER_SPRITE, SpriteInfo::new_default_size(80, 80));
        id_to_sprite_info.insert(SAND_SPRITE, SpriteInfo::new_default_size(0, 80));
        id_to_sprite_info.insert(GEM_FLOWER_SPRITE, SpriteInfo::new_default_size(160, 40));
        SpriteInfos { id_to_sprite_info }
    }
    pub fn load_entity_sprite_infos() -> Self {
        let mut id_to_sprite_info = HashMap::new();
        id_to_sprite_info.insert(
            KNIGHT_SPRITE_DOWN,
            SpriteInfo::new_all_data(5, 202, 27, 46, 40, 46, 13, 23),
        );
        id_to_sprite_info.insert(
            KNIGHT_SPRITE_UP,
            SpriteInfo::new_all_data(86, 204, 24, 46, 40, 46, 12, 23),
        );
        id_to_sprite_info.insert(
            KNIGHT_SPRITE_LEFT,
            SpriteInfo::new_all_data(120, 205, 30, 45, 40, 46, 12, 23),
        );
        id_to_sprite_info.insert(
            KNIGHT_SPRITE_RIGHT,
            SpriteInfo::new_all_data(160, 204, 40, 46, 40, 46, 12, 23),
        );
        id_to_sprite_info.insert(WOLF_SPRITE, SpriteInfo::new(200, 80, 40, 40));
        id_to_sprite_info.insert(MARKER_RUNE_SPRITE, SpriteInfo::new_default_size(120, 0));
        id_to_sprite_info.insert(ZOMBIE_SPRITE, SpriteInfo::new_default_size(80, 0));
        id_to_sprite_info.insert(NUN_SPRITE, SpriteInfo::new(40, 200, 40, 55));
        id_to_sprite_info.insert(VILLAGER_SPRITE, SpriteInfo::new(246, 1, 28, 39));
        id_to_sprite_info.insert(CROSS_SPRITE, SpriteInfo::new_default_size(360, 80));
        id_to_sprite_info.insert(
            TREE_SPRITE,
            SpriteInfo::new_all_data(1, 249, 35, 90, 35, 90, 18, 65),
        );
        id_to_sprite_info.insert(FIREBALL_SPRITE, SpriteInfo::new(251, 171, 16, 18));
        id_to_sprite_info.insert(CORPSE_SPRITE, SpriteInfo::new(280, 159, 40, 41));
        id_to_sprite_info.insert(NECROBOLT_SPRITE, SpriteInfo::new(56, 17, 8, 5));
        id_to_sprite_info.insert(SPELLBOOK_SPRITE, SpriteInfo::new_default_size(201, 204));
        id_to_sprite_info.insert(NECROMANCER_SPRITE, SpriteInfo::new(242, 204, 33, 43));
        id_to_sprite_info.insert(APPLE_SPRITE, SpriteInfo::new(90, 128, 20, 22));
        id_to_sprite_info.insert(WALL_SPRITE, SpriteInfo::new_default_size(40, 260));
        id_to_sprite_info.insert(CONFUSED_SPRITE, SpriteInfo::new_default_size(80, 260));
        id_to_sprite_info.insert(AMBUSH_SPRITE, SpriteInfo::new_default_size(80, 260));
        id_to_sprite_info.insert(DRAGON_SPRITE_RIGHT, SpriteInfo::new(1, 340, 109, 96));
        id_to_sprite_info.insert(DRAGON_SPRITE_LEFT, SpriteInfo::new(111, 340, 109, 96));
        id_to_sprite_info.insert(DRAGON_SPRITE_UP, SpriteInfo::new(221, 340, 96, 109));
        id_to_sprite_info.insert(DRAGON_SPRITE_DOWN, SpriteInfo::new(1, 437, 96, 109));
        id_to_sprite_info.insert(
            CREATURE_SPRITE_LEFT,
            SpriteInfo::new(319, 245, 42, 40).with_scaling(CREATURE_SCALING),
        );
        id_to_sprite_info.insert(
            CREATURE_SPRITE_RIGHT,
            SpriteInfo::new(276, 245, 42, 40).with_scaling(CREATURE_SCALING),
        );
        id_to_sprite_info.insert(
            CREATURE_SPRITE_UP,
            SpriteInfo::new(276, 204, 42, 40).with_scaling(CREATURE_SCALING),
        );
        id_to_sprite_info.insert(
            CREATURE_SPRITE_DOWN,
            SpriteInfo::new(319, 204, 42, 40).with_scaling(CREATURE_SCALING),
        );
        id_to_sprite_info.insert(FLAG_SPRITE, SpriteInfo::new(369, 215, 7, 19));
        id_to_sprite_info.insert(HOLY_SLASH_SPRITE, SpriteInfo::new(126, 265, 16, 50));
        id_to_sprite_info.insert(HOLY_SLASH_SPRITE_3, SpriteInfo::new(133, 261, 14, 67));
        id_to_sprite_info.insert(HOLY_SLASH_SPRITE_2, SpriteInfo::new(146, 261, 14, 67));
        id_to_sprite_info.insert(HOLY_SLASH_SPRITE_1, SpriteInfo::new(159, 261, 14, 67));
        id_to_sprite_info.insert(HOLY_BEAM_SPRITE, SpriteInfo::new(184, 326, 75, 5));
        id_to_sprite_info.insert(ANT_SPRITE, SpriteInfo::new(152, 272, 56, 24));
        id_to_sprite_info.insert(ANT_SPAWNER_SPRITE, SpriteInfo::new(111, 447, 98, 91));
        SpriteInfos { id_to_sprite_info }
    }
    pub fn ability_overlay_sprite_info() -> SpriteInfo {
        SpriteInfo::new(0, 0, 203, 95)
    }
    pub fn get(&self, id: u32) -> Option<SpriteInfo> {
        self.id_to_sprite_info
            .get(&id)
            .map(|sprite_info| sprite_info.clone())
    }
}
#[derive(Clone)]
/**
Where to find a sprite on a spritesheet, and how to then draw it at a certain point.
*/
pub struct SpriteInfo {
    pub x: i32,
    pub y: i32,
    // bounding box size in sprite file
    pub sprite_width: u32,
    pub sprite_height: u32,
    // The offset in the sprite file which is drawn at 0,0
    pub center_x: u32,
    pub center_y: u32,
    // The size of the entity, for purposes such as drawing health bars
    //     - may be larger than the sprite
    pub effective_width: u32,
    pub effective_height: u32,
    //How much larger the sprite should be on screen than in the spritesheet
    pub scaling: f64,
}
impl SpriteInfo {
    pub fn new_default_size(x: i32, y: i32) -> Self {
        Self::new(x, y, 40, 40)
    }
    pub fn new(x: i32, y: i32, sprite_width: u32, sprite_height: u32) -> Self {
        Self::new_all_data(
            x,
            y,
            sprite_width,
            sprite_height,
            sprite_width,
            sprite_height,
            sprite_width / 2,
            sprite_height / 2,
        )
    }
    /// Adds a manually set "center pixel", and how high and wide the sprite should be considered for effects such as healthbars
    pub fn new_all_data(
        x: i32,
        y: i32,
        sprite_width: u32,
        sprite_height: u32,
        effective_width: u32,
        effective_height: u32,
        center_x: u32,
        center_y: u32,
    ) -> Self {
        SpriteInfo {
            x,
            y,
            sprite_width,
            sprite_height,
            effective_width,
            effective_height,
            center_x,
            center_y,
            scaling: 1.0,
        }
    }
    pub fn with_scaling(mut self, scaling: f64) -> Self {
        self.scaling = scaling;
        self
    }
    ///Where the finished sprite goes on the screen
    pub fn get_sprite_dest_rect(
        &self,
        game: &Game,
        relative_coords: PixelCoords,
    ) -> sdl2::rect::Rect {
        let center_x = (self.center_x as f64 * self.scaling) as i32;
        let center_y = (self.center_y as f64 * self.scaling) as i32;
        let width = (self.sprite_width as f64 * self.scaling) as u32;
        let height = (self.sprite_height as f64 * self.scaling) as u32;
        let screen_coords = get_screen_coords(game, relative_coords);
        let dest_coords = screen_coords.translate(-center_x, -center_y);
        sdl2::rect::Rect::new(
            dest_coords.get_x().to_num(),
            dest_coords.get_y().to_num(),
            width,
            height,
        )
    }
    ///The rect representing the entity for purposes such as drawing healthbars
    pub fn get_effective_dest_rect(
        &self,
        game: &Game,
        relative_coords: PixelCoords,
    ) -> sdl2::rect::Rect {
        let screen_coords = get_screen_coords(game, relative_coords);
        let dest_coords = screen_coords.translate(
            -(self.effective_width as i32 / 2),
            -(self.effective_height as i32 / 2),
        );
        sdl2::rect::Rect::new(
            dest_coords.get_x().to_num(),
            dest_coords.get_y().to_num(),
            self.effective_width,
            self.effective_height,
        )
    }
    pub fn get_source_rect(&self) -> Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.sprite_width, self.sprite_height)
    }
}

pub fn get_screen_coords(game: &Game, relative_coords: PixelCoords) -> PixelCoords {
    let (viewport_width, viewport_height) = game.get_viewport_dimensions();
    relative_coords.stretch(1, -1)
        + PixelCoords::new_to_fixed(
            relative_coords.get_plane(),
            viewport_width / 2.0,
            viewport_height / 2.0,
        )
}
