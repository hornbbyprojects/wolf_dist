use super::*;

pub struct ResourceCollector {
    game_object_id: GameObjectId,
}

pub struct ResourceCollectorComponent {
    pub component_id: ComponentId,
    pub resource_collector_id: ResourceCollectorId,
}

impl Component for ResourceCollectorComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        ResourceCollector::remove(game, self.resource_collector_id);
    }
}

impl ResourceCollectorComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let resource_collector_id = ResourceCollector::new(game, owner_id);
        let comp = ResourceCollectorComponent {
            component_id,
            resource_collector_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}
impl ResourceCollector {
    pub fn step(game: &mut Game) {
        let mut to_pickup: Vec<(GameObjectId, Vec<GameObjectId>)> = Vec::new();
        {
            let collision_group =
                match CollisionSystem::get_collision_group(game, CollisionGroupId::Resource) {
                    Some(x) => x,
                    None => return,
                };
            let collision_map = collision_group.collision_map.borrow();
            for (_id, resource_collector) in game.resource_system.resource_collectors.iter() {
                let hit_box = resource_collector.game_object_id.get_hit_box(game);
                let pickups = collision_map
                    .get_colliding_game(game, hit_box)
                    .into_iter()
                    .collect();
                to_pickup.push((resource_collector.game_object_id, pickups));
            }
        }
        for (collector_game_object_id, pickups) in to_pickup {
            for pickup_game_object_id in pickups {
                pickup_game_object_id
                    .send_transfer_resources_signal(game, collector_game_object_id);
                pickup_game_object_id.remove(game);
            }
        }
    }
    pub fn new(game: &mut Game, game_object_id: GameObjectId) -> ResourceCollectorId {
        let id = game.get_id();
        game.resource_system
            .resource_collectors
            .insert(id, ResourceCollector { game_object_id });
        id
    }
    pub fn remove(game: &mut Game, id: ResourceCollectorId) {
        game.resource_system.resource_collectors.remove(id);
    }
}
