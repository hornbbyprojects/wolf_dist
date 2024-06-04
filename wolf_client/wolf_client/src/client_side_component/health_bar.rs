use super::*;
use crate::sprites::Sprites;
use sdl2::render::*;
use sdl2::video::*;

#[derive(Clone)]
pub struct HealthBarComponent {
    pub component_id: ComponentId,
}

impl DrawSignalListener for HealthBarComponent {
    fn clone_box(&self) -> Box<dyn DrawSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_draw_signal(
        &self,
        game: &Game,
        owner: GameObjectId,
        canvas: &mut Canvas<Window>,
        _sprites: &Sprites,
        _texture_creator: &TextureCreator<WindowContext>,
        dest_rect: sdl2::rect::Rect,
    ) {
        let health_proportion = owner
            .send_get_health_proportion_signal(game)
            .map(|x| x.extract())
            .unwrap_or(1.0);
        if health_proportion >= 0.999 {
            return;
        }
        let green_width = (dest_rect.width() as f64) * health_proportion;
        let new_y = dest_rect.y() + dest_rect.height() as i32;
        let green_dest_rect = sdl2::rect::Rect::new(dest_rect.x(), new_y, green_width as u32, 5);
        let red_dest_rect = sdl2::rect::Rect::new(dest_rect.x(), new_y, dest_rect.width(), 5);
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.fill_rect(red_dest_rect).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        canvas.fill_rect(green_dest_rect).unwrap();
    }
}

impl Component for HealthBarComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_draw_signal_listener(game, self.component_id);
    }
}

impl HealthBarComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, component_id: ComponentId) {
        let comp = HealthBarComponent { component_id };
        owner_id.add_component(game, comp.clone());
        owner_id.add_draw_signal_listener(game, comp);
    }
}
