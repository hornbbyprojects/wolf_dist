use super::*;

pub fn create_resource_pickup(game: &mut Game, coords: PixelCoords, mut resources: Resources) {
    if resources.is_empty() {
        return;
    }
    let game_object_id = GameObject::create_game(game, coords);
    ResourcePickupComponent::add_to(game, game_object_id);
    ResourceHolderComponent::add_to(game, game_object_id);
    BasicDrawingComponent::add_to(game, game_object_id, APPLE_SPRITE, DEFAULT_DEPTH);
    game_object_id.send_add_resources_signal(game, &mut resources);
}

#[derive(Clone)]
pub struct ResourcePickupComponent {
    component_id: ComponentId,
}

impl Component for ResourcePickupComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_collision_group(game, CollisionGroupId::Resource);
    }
}

impl ResourcePickupComponent {
    fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = ResourcePickupComponent { component_id };
        owner_id.add_collision_group(game, CollisionGroupId::Resource);
        owner_id.add_component(game, comp);
    }
}
