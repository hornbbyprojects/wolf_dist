use super::*;

pub struct Walker {
    pub max_speed: f64,
    pub dx: f64,
    pub dy: f64,
    pub acceleration: f64,
    pub game_object_id: GameObjectId,
    pub locomotion_mode_id: LocomotionModeId,
}

impl Walker {
    fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        max_speed: f64,
        acceleration: f64,
    ) -> WalkerId {
        let walker_id = game.get_id();
        let locomotion_mode_id = game.get_id();
        let walker = Walker {
            max_speed,
            dx: 0.0,
            dy: 0.0,
            locomotion_mode_id,
            game_object_id,
            acceleration,
        };
        game.movement_system
            .locomotion_system
            .walkers
            .insert(walker_id, walker);
        game_object_id.add_locomotion_mode_id(game, locomotion_mode_id);
        walker_id
    }
    pub fn step(game: &mut Game) {
        let mut update_facing = Vec::new();
        let mut to_update_velocity = Vec::new();
        for (id, walker) in game.movement_system.locomotion_system.walkers.iter_mut() {
            let game_object = game.game_objects.get(walker.game_object_id).unwrap();
            let mut should_walk = false;
            if let Some(locomotion_mode_selection) =
                game_object.components.get_locomotion_mode_selection()
            {
                if let Some(selected_mode) = locomotion_mode_selection.selected_locomotion_mode {
                    should_walk = selected_mode == walker.locomotion_mode_id;
                }
            }
            if should_walk {
                let intended_movement = game
                    .movement_system
                    .intend_move_system
                    .intended_movements
                    .get(&walker.game_object_id);
                {
                    let angle = match intended_movement {
                        Some(IntendedMovements::Confusion) => Some(Angle::enforce_range(
                            rand::thread_rng().gen_range(0.0..std::f64::consts::PI * 2.0),
                        )),
                        Some(IntendedMovements::MoveInDirection(angle)) => Some(angle.clone()),
                        Some(IntendedMovements::Follow(game_object_id)) => walker
                            .game_object_id
                            .get_direction_to_minimal(&game.game_objects, &game_object_id),
                        Some(IntendedMovements::MoveToPoint(point)) => Some(
                            walker
                                .game_object_id
                                .get_direction_to_point_minimal(&game.game_objects, point),
                        ),
                        None => None,
                    };
                    if let Some(angle) = angle {
                        let desired_vector = (
                            walker.max_speed * angle.cos(),
                            walker.max_speed * angle.sin(),
                        );
                        let acceleration_required =
                            (desired_vector.0 - walker.dx, desired_vector.1 - walker.dy);
                        let required_magnitude =
                            vector_magnitude(acceleration_required.0, acceleration_required.1);
                        let new_velocity = if required_magnitude < walker.acceleration {
                            desired_vector
                        } else {
                            (
                                walker.dx
                                    + acceleration_required.0 * walker.acceleration
                                        / required_magnitude,
                                walker.dy
                                    + acceleration_required.1 * walker.acceleration
                                        / required_magnitude,
                            )
                        };
                        let new_angle = vector_angle(new_velocity.0, new_velocity.1);
                        update_facing.push((walker.game_object_id, new_angle));
                        walker.dx = new_velocity.0;
                        walker.dy = new_velocity.1;
                        to_update_velocity.push((walker.game_object_id, new_velocity));
                    } else {
                        let current_velocity = vector_magnitude(walker.dx, walker.dy);
                        if current_velocity <= walker.acceleration {
                            walker.dx = 0.0;
                            walker.dy = 0.0;
                        } else {
                            let scaling =
                                (current_velocity - walker.acceleration) / current_velocity;
                            let new_velocity = (walker.dx * scaling, walker.dy * scaling);
                            walker.dx = new_velocity.0;
                            walker.dy = new_velocity.1;
                            to_update_velocity.push((walker.game_object_id, new_velocity));
                        }
                    }
                }
            }
        }
        for (game_object_id, angle) in update_facing {
            game_object_id.set_facing_sticky(game, angle);
        }
        for (game_object_id, (dx, dy)) in to_update_velocity {
            game_object_id.move_by_game(game, dx, dy)
        }
    }
    fn remove(game: &mut Game, walker_id: WalkerId) {
        let walker = game
            .movement_system
            .locomotion_system
            .walkers
            .remove(walker_id)
            .unwrap();
        let game_object = game.game_objects.get_mut(walker.game_object_id).unwrap();
        game_object.components.remove_locomotion_mode_selection();
    }
}

pub struct WalkerComponent {
    pub component_id: ComponentId,
    pub walker_id: WalkerId,
}

impl WalkerComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        max_speed: f64,
        acceleration: f64,
    ) -> ComponentId {
        let component_id = game.get_id();
        let walker_id = Walker::new(game, owner_id, max_speed, acceleration);
        let component = WalkerComponent {
            component_id,
            walker_id,
        };
        owner_id.add_component(game, component);
        component_id
    }
}

impl Component for WalkerComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        Walker::remove(game, self.walker_id)
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
