use super::*;
use crate::drawing::messages::draw_messages;
use crate::sprites::Sprites;
use crate::SCREEN_WIDTH;
use coords::PixelNum;
use sdl2::render::*;
use sdl2::video::*;

const MESSAGE_SPACING: i32 = 50;
const MESSAGE_X: i32 = SCREEN_WIDTH as i32 - 100;
impl Game {
    pub fn get_viewport_dimensions(&self) -> (f64, f64) {
        if let Some(bound_object_id) = self.current_bound_object {
            if self.game_objects.contains_key(bound_object_id) {
                let scale = bound_object_id
                    .send_get_vision_scale_signal(self)
                    .map(|x| x.extract())
                    .unwrap_or(1.0);
                let logical_width = crate::SCREEN_WIDTH as f64 * scale;
                let logical_height = crate::SCREEN_HEIGHT as f64 * scale;
                return (logical_width, logical_height);
            }
        }
        (crate::SCREEN_WIDTH as f64, crate::SCREEN_HEIGHT as f64)
    }
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        sprites: &mut Sprites,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let mut draw_order: Vec<(DrawableId, i32, PixelNum)> = Vec::new();
        for (drawable_id, drawable) in self.drawables.iter() {
            let negative_y_pos = -drawable.game_object_id.get_coords(self).get_y();
            let negative_depth = -drawable.depth;
            let index = match draw_order
                .binary_search_by_key(&(negative_depth, negative_y_pos), |(_id, d, y)| (*d, *y))
            {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            draw_order.insert(index, (drawable_id, negative_depth, negative_y_pos));
        }
        for (drawable_id, _, _) in draw_order {
            let drawable = self.drawables.get(drawable_id).unwrap();
            drawable.draw(self, canvas, sprites, texture_creator);
        }
        for (id, slash_animation) in self.slash_animations.iter() {
            slash_animation.draw(self, canvas, sprites, texture_creator);
        }
        draw_messages(canvas, texture_creator, &sprites, &self.notifications);
    }
}
