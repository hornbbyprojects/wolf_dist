use super::*;

pub struct ConstantVelocity {
    pub game_object_id: GameObjectId,
    pub x_speed: f64,
    pub y_speed: f64,
}

impl ConstantVelocity {
    pub fn step(game: &mut Game) {
        let mut to_move = Vec::new();
        for (_id, constant_velocity) in game.movement_system.constant_velocities.iter() {
            to_move.push((
                constant_velocity.game_object_id,
                constant_velocity.x_speed,
                constant_velocity.y_speed,
            ));
        }
        for (id, dx, dy) in to_move {
            id.move_by_game(game, dx, dy);
        }
    }
    fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        x_speed: f64,
        y_speed: f64,
    ) -> ConstantVelocityId {
        let id = game.get_id();
        let constant_velocity = ConstantVelocity {
            game_object_id,
            x_speed,
            y_speed,
        };
        game.movement_system
            .constant_velocities
            .insert(id, constant_velocity);
        id
    }
}

pub struct ConstantVelocityComponent {
    component_id: ComponentId,
    constant_velocity_id: ConstantVelocityId,
}

impl ConstantVelocityComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        x_speed: f64,
        y_speed: f64,
    ) -> ComponentId {
        let component_id = game.get_id();
        let constant_velocity_id = ConstantVelocity::new(game, owner_id, x_speed, y_speed);
        let comp = ConstantVelocityComponent {
            constant_velocity_id,
            component_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}
impl Component for ConstantVelocityComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        game.movement_system
            .constant_velocities
            .remove(self.constant_velocity_id);
    }
}
