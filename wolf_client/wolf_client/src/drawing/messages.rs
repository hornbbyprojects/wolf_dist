use sdl2::{
    rect::Rect,
    render::{Canvas, RenderTarget, TextureCreator},
    ttf::Font,
};

use crate::{sprite_info::SpriteInfo, SCREEN_WIDTH};

use super::*;

const MESSAGE_RIGHT_MARGIN: i32 = 50;
const MESSAGE_BOTTOM_MARGIN: i32 = 25;
pub fn draw_messages<'a, T: RenderTarget, U>(
    canvas: &mut Canvas<T>,
    texture_creator: &'a TextureCreator<U>,
    sprites: &Sprites,
    notifications: &Vec<Notification>,
) {
    let mut message_y = 100;
    for notification in notifications {
        // SDL2 doesn't have a nice way to do outlines in a different color, so we just draw it twice, once with an outline in black, once normally in white
        let partial_render = sprites.basic_font.render(&notification.message);
        let partial_render_outline = sprites.outline_font.render(&notification.message);
        let surface = partial_render
            .solid((255, 255, 255, 255))
            .expect("Unable to render text!");
        let surface_outline = partial_render_outline
            .solid((0, 0, 0, 255))
            .expect("Unable to render text!");
        let width = surface.width();
        let height = surface.height();
        let width_outline = surface_outline.width();
        let height_outline = surface_outline.height();
        let src = Rect::new(0, 0, width, height);
        let src_outline = Rect::new(0, 0, width_outline, height_outline);
        let dst = Rect::new(
            SCREEN_WIDTH as i32 - width_outline as i32 - MESSAGE_RIGHT_MARGIN,
            message_y,
            width,
            height,
        );
        let dst_outline = Rect::new(
            SCREEN_WIDTH as i32 - width_outline as i32 - MESSAGE_RIGHT_MARGIN,
            message_y,
            width_outline,
            height_outline,
        );
        let texture_outline = texture_creator
            .create_texture_from_surface(surface_outline)
            .unwrap();
        canvas
            .copy(&texture_outline, src_outline, dst_outline)
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        canvas.copy(&texture, src, dst).unwrap();
        message_y += height as i32 + MESSAGE_BOTTOM_MARGIN;
    }
}
