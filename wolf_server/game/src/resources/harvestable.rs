use super::*;
use crate::damage::DeathSignalListener;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(Harvest, &mut Game, harvester_id: GameObjectId);

pub struct Harvestable {
    pub game_object_id: GameObjectId,
    pub resources: Resources,
}

impl Harvestable {
    fn new(game: &mut Game, game_object_id: GameObjectId, resources: Resources) -> HarvestableId {
        let id = game.get_id();
        let harvestable = Harvestable {
            game_object_id,
            resources,
        };
        game.resource_system.harvestables.insert(id, harvestable);
        game_object_id.add_collision_group(game, CollisionGroupId::Harvestable);
        id
    }
    fn remove(game: &mut Game, id: HarvestableId) -> Option<Harvestable> {
        if let Some(harvestable) = game.resource_system.harvestables.remove(id) {
            harvestable
                .game_object_id
                .remove_collision_group(game, CollisionGroupId::Harvestable);
            Some(harvestable)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct HarvestableComponent {
    component_id: ComponentId,
    harvestable_id: HarvestableId,
}

impl HitBoxed for HarvestableId {
    fn get_hit_box(&self, game: &Game) -> HitBox {
        let game_object_id = game
            .resource_system
            .harvestables
            .get(*self)
            .unwrap()
            .game_object_id;
        game_object_id.get_hit_box(game)
    }
}

impl DeathSignalListener for HarvestableComponent {
    fn clone_box(&self) -> Box<(dyn DeathSignalListener + 'static)> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_death_signal(&self, game: &mut Game, owner_id: GameObjectId) {
        let harvestable = game
            .resource_system
            .harvestables
            .get_mut(self.harvestable_id)
            .unwrap();
        let coords = harvestable.game_object_id.get_coords(&game.game_objects);
        let resources = std::mem::replace(&mut harvestable.resources, Resources::new());
        create_resource_pickup(game, coords, resources);
    }
}

impl Component for HarvestableComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_harvest_signal_listener(game, self.component_id);
        owner_id.remove_death_signal_listener(game, self.component_id);
        Harvestable::remove(game, self.harvestable_id);
    }
}

impl HarvestSignalListener for HarvestableComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn HarvestSignalListener> {
        Box::new(self.clone())
    }
    fn receive_harvest_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        harvester_id: GameObjectId,
    ) {
        let harvestable = game
            .resource_system
            .harvestables
            .get_mut(self.harvestable_id)
            .unwrap();
        let mut resources = std::mem::replace(&mut harvestable.resources, Resources::new());
        harvester_id.send_add_resources_signal(game, &mut resources);
        owner_id.remove(game);
    }
}

impl HarvestableComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, resources: Resources) {
        let component_id = game.get_id();
        let harvestable_id = Harvestable::new(game, owner_id, resources);
        let comp = HarvestableComponent {
            component_id,
            harvestable_id,
        };
        owner_id.add_harvest_signal_listener(game, comp.clone());
        owner_id.add_death_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
    }
}
