use super::*;
use crate::drawing::speech::draw_speech;
use crate::sprites::Sprites;
use sdl2::rect::Rect;
use sdl2::render::*;

const SPEECH_OFFSET_Y: i32 = -13;
const SPEECH_OFFSET_X: i32 = 0;

pub fn draw_speech_for_id<S, T: RenderTarget>(
    game: &Game,
    id: GameObjectId,
    canvas: &mut Canvas<T>,
    texture_creator: &TextureCreator<S>,
    sprites: &Sprites,
    draw_rect: &Rect,
) {
    if let Some((_component_id, current)) = game.currently_saying.get(id) {
        let center_x = draw_rect.x + draw_rect.width() as i32 / 2;
        draw_speech(
            canvas,
            texture_creator,
            &sprites.basic_font,
            current,
            center_x + SPEECH_OFFSET_X,
            draw_rect.y + SPEECH_OFFSET_Y,
        );
    }
}
