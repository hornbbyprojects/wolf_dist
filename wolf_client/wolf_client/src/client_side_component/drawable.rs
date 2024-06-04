use super::*;
use crate::sprites::Sprites;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::video::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(
    Draw,
    &Game,
    canvas: &mut Canvas<Window>,
    sprites: &Sprites,
    texture_creator: &TextureCreator<WindowContext>,
    dest_rect: sdl2::rect::Rect
);
define_signal_listener!(MutateTexture, &Game, texture: &mut Texture);

pub struct Drawable {
    pub game_object_id: GameObjectId,
    pub sprite: u32,
    pub depth: i32,
}

#[derive(Clone)]
pub struct DrawableComponent {
    pub component_id: ComponentId,
    pub drawable_id: DrawableId,
}

impl Drawable {
    pub fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        sprite: u32,
        depth: i32,
    ) -> DrawableId {
        let drawable_id = game.get_id();
        let drawable = Drawable {
            game_object_id,
            sprite,
            depth,
        };
        game.drawables.insert(drawable_id, drawable);
        drawable_id
    }
    pub fn remove(game: &mut Game, drawable_id: DrawableId) {
        game.drawables.remove(drawable_id);
    }
}

impl DrawableComponent {
    pub fn add_to(
        game: &mut Game,
        game_object_id: GameObjectId,
        component_id: ComponentId,
        sprite: u32,
        depth: i32,
    ) {
        let drawable_id = Drawable::new(game, game_object_id, sprite, depth);
        let comp = DrawableComponent {
            component_id,
            drawable_id,
        };
        game_object_id.add_component(game, comp);
    }
}

impl Component for DrawableComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        Drawable::remove(game, self.drawable_id);
    }
}

impl Drawable {
    pub fn draw(
        &self,
        game: &Game,
        canvas: &mut Canvas<Window>,
        sprites: &Sprites,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        Drawable::draw_sprite_for_game_object(
            game,
            canvas,
            sprites,
            texture_creator,
            self.game_object_id,
            self.sprite,
        )
    }
    pub fn draw_sprite_for_game_object(
        game: &Game,
        canvas: &mut Canvas<Window>,
        sprites: &Sprites,
        texture_creator: &TextureCreator<WindowContext>,
        game_object_id: GameObjectId,
        sprite: u32,
    ) {
        let effective_dest_rect = {
            let game_object = game.game_objects.get(game_object_id).unwrap();
            if game_object.coords.get_plane() != game.current_view_coords.get_plane() {
                // Visions of another world...
                return;
            }

            let sprite_info = match sprites.entity_sprite_infos.get(sprite) {
                Some(x) => x,
                None => panic!("Invalid sprite id {}", sprite),
            };

            let coords = game_object.coords - game.current_view_coords;
            let source_rect = sprite_info.get_source_rect();
            let dest_rect = sprite_info.get_sprite_dest_rect(game, coords);
            let dest_rect_no_offset = Rect::new(0, 0, dest_rect.width(), dest_rect.height());
            let effective_dest_rect = sprite_info.get_effective_dest_rect(game, coords);

            let mut small_texture = texture_creator
                .create_texture(
                    Some(PixelFormatEnum::RGBA8888),
                    TextureAccess::Target,
                    dest_rect.width(),
                    dest_rect.height(),
                )
                .unwrap();

            canvas.with_texture_canvas(&mut small_texture, |canvas| {
                canvas.set_draw_color(Color::RGBA(255, 255, 255, 0));
                canvas.set_blend_mode(BlendMode::None);
                canvas.clear();
                canvas
                    .copy_ex(
                        &sprites.main_sprite_sheet,
                        source_rect,
                        dest_rect_no_offset,
                        0.0,
                        None, //rotate around centre
                        false,
                        false,
                    )
                    .unwrap();
            });

            small_texture.set_blend_mode(BlendMode::Blend);
            game_object_id.send_mutate_texture_signal(game, &mut small_texture);
            let rotation_in_degrees = game_object.rotation.degrees();
            canvas
                .copy_ex(
                    &small_texture,
                    dest_rect_no_offset,
                    dest_rect,
                    360.0 - rotation_in_degrees, // we go counterclockwise, SDL2 goes clockwise
                    None,                        //rotate around centre
                    false,
                    false,
                )
                .unwrap();

            effective_dest_rect
        };
        draw_speech_for_id(
            game,
            game_object_id,
            canvas,
            texture_creator,
            sprites,
            &effective_dest_rect,
        );
        game_object_id.send_draw_signal(
            game,
            canvas,
            sprites,
            texture_creator,
            effective_dest_rect,
        );
    }
}
