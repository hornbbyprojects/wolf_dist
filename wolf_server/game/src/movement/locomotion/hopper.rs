use super::*;

const HOP_PROGRESS_PER_TICK: f64 = 0.02;
const HOP_HEIGHT: f64 = 36.0;

pub struct Hop {
    start_position: PixelCoords,
    end_position: PixelCoords,
    progress: f64,
}

impl Hop {
    fn from_angle(start_position: PixelCoords, angle: Angle, length: f64) -> Self {
        Hop {
            start_position,
            end_position: start_position.offset_direction(angle, length),
            progress: 0.0,
        }
    }
}

pub struct Hopper {
    pub speed: f64,
    pub game_object_id: GameObjectId,
    pub locomotion_mode_id: LocomotionModeId,
    pub current_hop: Option<Hop>,
}

impl Hopper {
    fn new(game: &mut Game, game_object_id: GameObjectId, speed: f64) -> HopperId {
        let hopper_id = game.get_id();
        let locomotion_mode_id = game.get_id();
        let hopper = Hopper {
            speed,
            locomotion_mode_id,
            game_object_id,
            current_hop: None,
        };
        game.movement_system
            .locomotion_system
            .hoppers
            .insert(hopper_id, hopper);
        game_object_id.add_locomotion_mode_id(game, locomotion_mode_id);
        hopper_id
    }
    pub fn step(game: &mut Game) {
        let mut to_move = Vec::new();
        let mut to_sticky_face = Vec::new();
        let mut to_stop_sticky_face = Vec::new();
        for (_hopper_id, hopper) in game.movement_system.locomotion_system.hoppers.iter_mut() {
            let game_object = game.game_objects.get(hopper.game_object_id).unwrap();
            let start_position = game_object.coords.clone();
            let mut should_hop = false;
            if hopper.current_hop.is_none() {
                if let Some(locomotion_mode_selection) =
                    game_object.components.get_locomotion_mode_selection()
                {
                    if let Some(selected_mode) = locomotion_mode_selection.selected_locomotion_mode
                    {
                        should_hop = selected_mode == hopper.locomotion_mode_id;
                    }
                }
            }
            if should_hop {
                if let Some(intended_movement) = game
                    .movement_system
                    .intend_move_system
                    .intended_movements
                    .get(&hopper.game_object_id)
                {
                    let angle = match intended_movement {
                        IntendedMovements::Confusion => Some(Angle::enforce_range(
                            rand::thread_rng().gen_range(0.0..std::f64::consts::PI * 2.0),
                        )),
                        IntendedMovements::MoveInDirection(angle) => Some(angle.clone()),
                        IntendedMovements::Follow(game_object_id) => hopper
                            .game_object_id
                            .get_direction_to_minimal(&game.game_objects, game_object_id),
                        IntendedMovements::MoveToPoint(point) => Some(
                            hopper
                                .game_object_id
                                .get_direction_to_point_minimal(&game.game_objects, point),
                        ),
                    };
                    if let Some(angle) = angle {
                        to_sticky_face.push((hopper.game_object_id, angle));
                        let hop = Hop::from_angle(start_position, angle, 40.0 * hopper.speed);
                        hopper.current_hop = Some(hop);
                    }
                }
            }

            if let Some(ref mut current_hop) = hopper.current_hop {
                current_hop.progress = 1.0f64.min(current_hop.progress + HOP_PROGRESS_PER_TICK);
                let initial_speed = HOP_HEIGHT * 4.0;
                let gravity = initial_speed * 2.0;
                let height =
                    current_hop.progress * (initial_speed - 0.5 * gravity * current_hop.progress);
                let new_position = (current_hop.start_position * (1.0 - current_hop.progress))
                    + (current_hop.end_position * current_hop.progress)
                    + PixelCoords::new_to_fixed(
                        current_hop.start_position.get_plane(),
                        0.0,
                        height,
                    );
                to_move.push((hopper.game_object_id, new_position));
                if current_hop.progress == 1.0 {
                    hopper.current_hop = None;
                    to_stop_sticky_face.push(hopper.game_object_id);
                }
            }
        }
        for game_object_id in to_stop_sticky_face {
            game_object_id.stop_facing_sticky(game);
        }
        for (game_object_id, angle) in to_sticky_face {
            game_object_id.set_facing_sticky(game, angle);
        }
        for (game_object_id, position) in to_move {
            game_object_id.move_to_game(game, position);
        }
    }
    fn remove(game: &mut Game, hopper_id: HopperId) {
        let hopper = game
            .movement_system
            .locomotion_system
            .hoppers
            .remove(hopper_id)
            .unwrap();
        hopper
            .game_object_id
            .remove_locomotion_mode_id(game, hopper.locomotion_mode_id);
    }
}

pub struct HopperComponent {
    pub component_id: ComponentId,
    pub hopper_id: HopperId,
}

impl HopperComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, speed: f64) -> ComponentId {
        let component_id = game.get_id();
        let hopper_id = Hopper::new(game, owner_id, speed);
        let component = HopperComponent {
            component_id,
            hopper_id,
        };
        owner_id.add_component(game, component);
        component_id
    }
}

impl Component for HopperComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        Hopper::remove(game, self.hopper_id)
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
