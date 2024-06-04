use crate::combinable::OrBool;
use crate::game::*;
use crate::terrain::get_chunk_index_from_relative_coords;

pub struct SolidSystem {
    blockable_movers: IdMap<BlockableMoverId, BlockableMover>,
}

#[derive(Clone)]
pub struct SolidComponent {
    component_id: ComponentId,
}

impl SolidComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = SolidComponent { component_id };
        owner_id.add_collision_group(game, CollisionGroupId::Solid);
        owner_id.add_component(game, comp);
    }
}
impl Component for SolidComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_collision_group(game, CollisionGroupId::Solid);
    }
}

impl SolidSystem {
    pub fn new() -> Self {
        SolidSystem {
            blockable_movers: IdMap::new(),
        }
    }
    pub fn pre_movement(game: &mut Game) {
        BlockableMover::pre_movement(game);
    }
    pub fn post_movement(game: &mut Game) {
        BlockableMover::post_movement(game);
    }
}

struct BlockableMover {
    game_object_id: GameObjectId,
    blocked: bool,
}

impl BlockableMover {
    fn pre_movement(game: &mut Game) {
        let mut to_block = Vec::new();
        {
            let collision_group =
                CollisionSystem::get_collision_group(game, CollisionGroupId::Solid);
            let collision_map = collision_group.map(|x| x.collision_map.borrow());
            for (id, blockable_mover) in game.solid_system.blockable_movers.iter() {
                if game
                    .movement_system
                    .intend_move_system
                    .mounts_by_mounter
                    .contains_key(&blockable_mover.game_object_id)
                {
                    continue; // Ignore collision while mounted
                }
                if let Some(target_coords) = game
                    .movement_system
                    .to_move
                    .get(&blockable_mover.game_object_id)
                {
                    let hit_box = blockable_mover
                        .game_object_id
                        .get_hit_box(game)
                        .move_to(target_coords.clone());
                    let mut blocked = false;
                    for square in hit_box.get_overlapping_squares() {
                        let chunk_coords = square.into();
                        let relative = square.relative_to_chunk(chunk_coords).unwrap();
                        if let Some(chunk) = game.terrain.chunks.get(&chunk_coords) {
                            let index = get_chunk_index_from_relative_coords(relative);
                            if chunk.chunk_squares[index].is_solid() {
                                blocked = true;
                                break;
                            }
                        }
                    }
                    if !blocked {
                        if let Some(ref collision_map) = collision_map {
                            let colliding = collision_map.get_colliding_game(game, hit_box);
                            if !colliding.is_empty() {
                                blocked = true;
                            }
                        }
                    }
                    if blocked {
                        to_block.push(id);
                    }
                }
            }
        }
        for id in to_block {
            let blockable_mover = game.solid_system.blockable_movers.get_mut(id).unwrap();
            blockable_mover.blocked = true;
        }
    }
    fn post_movement(game: &mut Game) {
        for (_id, blockable_mover) in game.solid_system.blockable_movers.iter_mut() {
            blockable_mover.blocked = false;
        }
    }
    fn new(game: &mut Game, game_object_id: GameObjectId) -> BlockableMoverId {
        let blockable_mover_id = game.get_id();
        let blockable_mover = BlockableMover {
            game_object_id,
            blocked: false,
        };
        game.solid_system
            .blockable_movers
            .insert(blockable_mover_id, blockable_mover);
        blockable_mover_id
    }
    fn remove(game: &mut Game, id: BlockableMoverId) {
        game.solid_system.blockable_movers.remove(id);
    }
}

#[derive(Clone)]
pub struct BlockableMoverComponent {
    component_id: ComponentId,
    blockable_mover_id: BlockableMoverId,
}

impl Component for BlockableMoverComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_block_movement_signal_listener(game, self.component_id);
        BlockableMover::remove(game, self.blockable_mover_id)
    }
}

impl BlockableMoverComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let blockable_mover_id = BlockableMover::new(game, owner_id);
        let comp = BlockableMoverComponent {
            component_id,
            blockable_mover_id,
        };
        owner_id.add_get_block_movement_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}

impl GetBlockMovementSignalListener for BlockableMoverComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetBlockMovementSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_block_movement_signal(&self, game: &Game, _owner_id: GameObjectId) -> OrBool {
        let blockable_mover = game
            .solid_system
            .blockable_movers
            .get(self.blockable_mover_id)
            .unwrap();
        OrBool(blockable_mover.blocked)
    }
}
