use crate::abilities::SpellbookAbsorberComponent;
use crate::characters::*;
use crate::damage::DamageableComponent;
use crate::game::*;
use crate::terrain::{BasicChunkLoaderComponent, ChunkWatcherComponent};

impl Game {
    pub fn create_basic_body(&mut self) -> GameObjectId {
        let starting_coords = PixelCoords::new_at_zero();
        let object_to_watch = GameObject::create_game(self, starting_coords);

        BasicChunkLoaderComponent::add_to(self, object_to_watch);

        ChunkWatcherComponent::add_to(self, object_to_watch);

        DamageableComponent::add_to(self, object_to_watch);

        SpellbookAbsorberComponent::add_to(self, object_to_watch);

        PaladinCharacterComponent::add_to(self, object_to_watch);

        object_to_watch
    }
}
