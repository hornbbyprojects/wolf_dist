use sdl2::{
    rect::Rect,
    render::{Canvas, RenderTarget, TextureCreator},
    ttf::Font,
};

use super::*;

pub fn draw_speech<'a, T: RenderTarget, U>(
    canvas: &mut Canvas<T>,
    texture_creator: &'a TextureCreator<U>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
) {
    let partial_render = font.render(text);
    let surface = partial_render
        .solid((255, 255, 255, 255))
        .expect("Unable to render text!");
    let src = Rect::new(0, 0, surface.width(), surface.height());
    let dst = Rect::new(
        x - surface.width() as i32 / 2,
        y,
        surface.width(),
        surface.height(),
    );
    let texture = texture_creator
        .create_texture_from_surface(surface)
        .unwrap();
    canvas.copy(&texture, src, dst).unwrap();
}
