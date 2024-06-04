use crate::terrain::TerrainSpriteComponent;

use super::*;

pub enum ScaffoldType {
    Wall,
}
pub struct Scaffold {
    scaffold_type: ScaffoldType,
    game_object_id: GameObjectId,
}

impl Scaffold {
    pub fn create(game: &mut Game, coords: SquareCoords) {
        let id = game.get_id();
        let owner_id = GameObject::create_game(game, coords.center_pixel());
        TerrainSpriteComponent::add_to(game, owner_id, coords, SCAFFOLD_SPRITE);
        DamageableComponent::add_to(game, owner_id);
        let scaffold = Scaffold {
            scaffold_type: ScaffoldType::Wall,
            game_object_id: owner_id,
        };
        game.villages_system.scaffolds.insert(id, scaffold);
        game.villages_system.unassigned_scaffolds.insert(id);
    }
    pub fn build(game: &mut Game, id: ScaffoldId) {
        if let Some(scaffold) = game.villages_system.scaffolds.remove(id) {
            if let Some(coords) = scaffold.game_object_id.get_coords_safe(&game.game_objects) {
                create_wall(game, coords.into());
                scaffold.game_object_id.remove(game);
            }
            Scaffold::remove(game, id);
        }
    }
    pub fn remove(game: &mut Game, id: ScaffoldId) {
        game.villages_system.scaffolds.remove(id);
        game.villages_system.unassigned_scaffolds.remove(&id);
    }
}

pub struct BuildScaffoldBehaviour {
    mind_id: MindId,
    target: Option<(ScaffoldId, GameObjectId)>,
}

const SCAFFOLD_BUILD_DISTANCE: f64 = 50.0;
impl BuildScaffoldBehaviour {
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let id = game.get_id();
        let behaviour = BuildScaffoldBehaviour {
            mind_id,
            target: None,
        };
        game.villages_system
            .build_scaffold_behaviours
            .insert(id, behaviour);
        id
    }
    pub fn activate(game: &mut Game, id: BehaviourId, scaffold_id: ScaffoldId) {
        let target_game_object_id = game
            .villages_system
            .scaffolds
            .get(scaffold_id)
            .unwrap()
            .game_object_id;
        game.villages_system
            .build_scaffold_behaviours
            .get_mut(id)
            .unwrap()
            .target = Some((scaffold_id, target_game_object_id));
        game.villages_system
            .unassigned_scaffolds
            .remove(&scaffold_id);
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        if let Some(behaviour) = game.villages_system.build_scaffold_behaviours.remove(id) {
            if let Some((scaffold_id, _)) = behaviour.target {
                if game.villages_system.scaffolds.contains_key(scaffold_id) {
                    game.villages_system
                        .unassigned_scaffolds
                        .insert(scaffold_id);
                }
            }
        }
    }
    pub fn step(game: &mut Game) {
        let mut to_build = Vec::new();
        let mut to_say_start = Vec::new();
        let mut to_say_build = Vec::new();
        for (id, behaviour) in game.villages_system.build_scaffold_behaviours.iter_mut() {
            let mind = game
                .behaviour_system
                .minds
                .get_mut(behaviour.mind_id)
                .unwrap();
            if mind.active_behaviour != Some(id) {
                continue;
            }
            let my_coords = mind.game_object_id.get_coords(&game.game_objects);
            if let Some((target, target_game_object_id)) = behaviour.target {
                if let Some(target_coords) =
                    target_game_object_id.get_coords_safe(&game.game_objects)
                {
                    if my_coords.get_distance_to(&target_coords) < SCAFFOLD_BUILD_DISTANCE {
                        to_say_build.push(mind.game_object_id);
                        to_build.push(target);
                        behaviour.target = None;
                        mind.active_behaviour = None;
                    } else {
                        to_say_start.push(mind.game_object_id);
                        mind.game_object_id.intend_move_to_point(
                            &mut game.movement_system.intend_move_system,
                            target_coords,
                        );
                    }
                } else {
                    mind.active_behaviour = None;
                }
            } else {
                panic!("Scaffold behaviour with no scaffold targeted");
            }
        }
        for id in to_say_build {
            id.speak_safe(game, "TOK TOK TOK", Some(game.tick_counter + 100));
        }
        for id in to_say_start {
            id.speak_safe(
                game,
                "Time to build scaffold!",
                Some(game.tick_counter + 100),
            );
        }
        for id in to_build {
            Scaffold::build(game, id);
        }
    }
}
