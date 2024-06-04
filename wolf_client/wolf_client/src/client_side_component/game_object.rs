use super::*;
use anymap::AnyMap;
use coords::*;
use id::IdMap;

pub struct GameObject {
    pub coords: PixelCoords,
    pub rotation: Angle,
    pub listeners: AnyMap,
    pub components: IdMap<ComponentId, Box<dyn Component>>,
}

impl GameObject {
    pub fn new() -> Self {
        GameObject {
            coords: PixelCoords::new_at_zero(),
            rotation: Angle::zero(),
            listeners: AnyMap::new(),
            components: IdMap::new(),
        }
    }
}

impl GameObjectId {
    //silently returns 0 square on failure
    pub fn get_coords(&self, game: &Game) -> PixelCoords {
        game.game_objects
            .get(*self)
            .map(|x| x.coords)
            .unwrap_or(PixelCoords::new_at_zero())
    }
    pub fn get_square_coords(&self, game: &Game) -> SquareCoords {
        self.get_coords(game).into()
    }
    pub fn get_chunk_coords(&self, game: &Game) -> TerrainChunkCoords {
        self.get_coords(game).into()
    }
    pub fn get_distance_to_point(&self, game: &Game, point: &PixelCoords) -> f64 {
        let my_coords = self.get_coords(game);
        my_coords.get_distance_to(point)
    }
    pub fn get_distance_to(&self, game: &Game, other: &GameObjectId) -> f64 {
        let their_coords = other.get_coords(game);
        self.get_distance_to_point(game, &their_coords)
    }
    pub fn get_direction_to(&self, game: &Game, other: &GameObjectId) -> Angle {
        let my_coords = self.get_coords(game);
        let their_coords = other.get_coords(game);
        my_coords.get_direction_to(&their_coords)
    }
    pub fn add_component<T: Component + 'static>(&self, game: &mut Game, component: T) {
        self.remove_component(game, component.get_component_id());
        let this = game
            .game_objects
            .get_mut(*self)
            .expect("Can't add component to deleted object");
        this.components
            .insert(component.get_component_id(), Box::new(component));
    }
    //does nothing if component not present
    pub fn remove_component(&self, game: &mut Game, component_id: ComponentId) {
        let component = {
            let game_object = game.game_objects.get_mut(*self).unwrap();
            let component = game_object.components.remove(component_id);
            if let Some(component) = component {
                component
            } else {
                return;
            }
        };
        component.on_remove(game, *self);
    }
    pub fn set_rotation(&self, game: &mut Game, rotation: Angle) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        game_object.rotation = rotation;
    }
    pub fn remove(&self, game: &mut Game) {
        let components = {
            let game_object = game.game_objects.get_mut(*self).unwrap();
            std::mem::replace(&mut game_object.components, IdMap::new())
        };
        for (_id, component) in components {
            component.on_remove(game, *self);
        }
        game.game_objects.remove(*self);
    }
}
