use super::*;
use crate::generic::RemoveComponentAtComponent;
pub const ARC_TIME: u32 = 25;
pub const GRAVITY: f64 = 0.49;

#[derive(Debug, Clone)]
pub struct ArcingComponent {
    component_id: ComponentId,
    constant_velocity_component_id: ComponentId,
    remove_component_after_id: ComponentId,
}

impl ArcingComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, target_coords: PixelCoords) {
        let component_id = game.get_id();
        let starting_coords = owner_id.get_coords_game(game);
        let diff = target_coords - starting_coords;
        let dx = diff.get_x();
        let dy = diff.get_y();
        let x_speed = dx / PixelNum::from_num(ARC_TIME);
        let y_speed = dy / PixelNum::from_num(ARC_TIME);
        let constant_velocity_component_id =
            ConstantVelocityComponent::add_to(game, owner_id, x_speed.to_num(), y_speed.to_num());
        let remove_component_after_id = RemoveComponentAtComponent::add_to(
            game,
            owner_id,
            game.tick_counter + ARC_TIME,
            component_id,
        );
        let comp = ArcingComponent {
            component_id,
            constant_velocity_component_id,
            remove_component_after_id,
        };
        owner_id.add_component(game, comp);
    }
}

impl Component for ArcingComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_component(game, self.constant_velocity_component_id);
        owner_id.remove_component(game, self.remove_component_after_id);
    }
}
