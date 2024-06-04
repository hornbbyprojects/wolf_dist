use crate::game::*;
use crate::monsters::add_zombie;

pub struct Necrobolt {
    game_object_id: GameObjectId,
}
impl Necrobolt {
    fn new(game: &mut Game, game_object_id: GameObjectId) -> NecroboltId {
        let id = game.get_id();
        let necrobolt = Necrobolt { game_object_id };
        game.necromancy_system.necrobolts.insert(id, necrobolt);
        id
    }
    fn remove(game: &mut Game, id: NecroboltId) {
        game.necromancy_system.necrobolts.remove(id);
    }
    pub fn step(game: &mut Game) {
        let mut zombie_spawns = Vec::new();
        let mut to_remove = Vec::new();
        {
            let collision_group =
                match CollisionSystem::get_collision_group(game, CollisionGroupId::Corpse) {
                    Some(x) => x,
                    None => return,
                };
            let collision_map = collision_group.collision_map.borrow();
            for (_id, necrobolt) in game.necromancy_system.necrobolts.iter() {
                let hit_box = necrobolt.game_object_id.get_hit_box(game);
                let to_ressurect = collision_map.get_colliding_game(game, hit_box);
                for game_object_id in to_ressurect {
                    let coords = game_object_id.get_coords_game(game);
                    zombie_spawns.push(coords);
                    to_remove.push(game_object_id);
                }
            }
        }
        for coords in zombie_spawns {
            add_zombie(game, coords);
        }
        for game_object_id in to_remove {
            game_object_id.remove(game);
        }
    }
}
pub struct NecroboltComponent {
    component_id: ComponentId,
    necrobolt_id: NecroboltId,
}

impl NecroboltComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let necrobolt_id = Necrobolt::new(game, owner_id);
        let comp = NecroboltComponent {
            component_id,
            necrobolt_id,
        };
        owner_id.add_component(game, comp);
    }
}

impl Component for NecroboltComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        Necrobolt::remove(game, self.necrobolt_id);
    }
}
