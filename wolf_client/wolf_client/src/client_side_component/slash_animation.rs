use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};
use sprite_mappings::*;

use crate::sprites::Sprites;

use super::*;

pub struct SlashAnimation {
    game_object_id: GameObjectId,
    start_tick: u32,
}

const ANIMATION_SPEED: u32 = 2;

impl SlashAnimation {
    pub fn draw(
        &self,
        game: &Game,
        canvas: &mut Canvas<Window>,
        sprites: &Sprites,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let frames = [HOLY_SLASH_SPRITE];
        let ticks_since = game.tick_counter - self.start_tick;
        let frames_since = (ticks_since / ANIMATION_SPEED).min(frames.len() as u32 - 1);
        let sprite = frames[frames_since as usize];
        Drawable::draw_sprite_for_game_object(
            game,
            canvas,
            sprites,
            texture_creator,
            self.game_object_id,
            sprite,
        );
    }
    pub fn new(game: &mut Game, game_object_id: GameObjectId, start_tick: u32) -> SlashAnimationId {
        let id = game.get_id();
        let slash_animation = SlashAnimation {
            game_object_id,
            start_tick,
        };
        game.slash_animations.insert(id, slash_animation);
        id
    }
}

pub struct SlashAnimationComponent {
    component_id: ComponentId,
    slash_animation_id: SlashAnimationId,
}
impl Component for SlashAnimationComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }

    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        game.slash_animations.remove(self.slash_animation_id);
    }
}
impl SlashAnimationComponent {
    pub fn add_to(
        game: &mut Game,
        game_object_id: GameObjectId,
        component_id: ComponentId,
        start_tick: u32,
    ) {
        let slash_animation_id = SlashAnimation::new(game, game_object_id, start_tick);
        let component = SlashAnimationComponent {
            component_id,
            slash_animation_id,
        };
        game_object_id.add_component(game, component);
    }
}
