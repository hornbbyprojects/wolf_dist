use crate::component::*;
use crate::game::{CollisionGroupId, Game};
use crate::id_types::*;
use crate::spatial_map::{HitBox, HitBoxed};
use anymap::AnyMap;
use coords::*;
use id::*;
use signal_listener_macro::define_signal_listener;
use wolf_hash_map::*;

mod client_side_component;
pub use client_side_component::*;

define_signal_listener!(BeforeDelete, &mut Game);

pub struct GameObject {
    pub coords: PixelCoords,
    pub rotation: Angle,
    pub hit_box: HitBox,
    pub listeners: AnyMap,
    pub components: GameObjectComponents,
    pub collision_groups: WolfHashMap<CollisionGroupId, u32>,
    pub players_sent_to_last_tick: WolfHashSet<PlayerId>,
    pub players_sent_to_this_tick: WolfHashSet<PlayerId>,
    pub client_side_components: IdMap<ClientSideComponentId, ClientSideComponent>,
    pub last_sent_client_side_components: WolfHashSet<ClientSideComponentId>,
}

impl GameObject {
    pub fn create_game(game: &mut Game, starting_coords: PixelCoords) -> GameObjectId {
        let id = game.get_id();
        let object = GameObject::new(starting_coords);
        game.game_objects.insert(id, object);
        id
    }
    pub fn create_with_hit_box(
        game: &mut Game,
        starting_coords: PixelCoords,
        hit_box: HitBox,
    ) -> GameObjectId {
        let id = game.get_id();
        let mut object = GameObject::new(starting_coords);
        object.hit_box = hit_box;
        game.game_objects.insert(id, object);
        id
    }
    pub fn new(starting_coords: PixelCoords) -> Self {
        GameObject {
            coords: starting_coords,
            rotation: Angle::assume_in_range(0.0),
            listeners: AnyMap::new(),
            components: GameObjectComponents::new(),
            hit_box: HitBox::default(),
            collision_groups: WolfHashMap::new(),
            players_sent_to_last_tick: WolfHashSet::new(),
            players_sent_to_this_tick: WolfHashSet::new(),
            client_side_components: IdMap::new(),
            last_sent_client_side_components: WolfHashSet::new(),
        }
    }
    pub fn _remove(game: &mut Game, id: GameObjectId) {
        id.send_before_delete_signal(game);
        let game_object = match game.game_objects.get_mut(id) {
            Some(x) => x,
            None => return,
        };
        let components = std::mem::replace(&mut game_object.components.components, IdMap::new());
        for (_component_id, component) in components {
            component.on_remove(game, id);
        }
        if let Some(collision_group) = game
            .collision_system
            .collision_groups
            .get(CollisionGroupId::ClientSideComponent)
        {
            collision_group.collision_map.borrow_mut().remove(id);
        }

        game.game_objects.remove(id);
    }
    pub fn get_hit_box(&self) -> HitBox {
        self.hit_box
            .clone()
            .rotate(self.rotation)
            .translate(self.coords)
    }
}

impl GameObjectId {
    pub fn get<'a>(&self, game: &'a Game) -> Option<&'a GameObject> {
        game.game_objects.get(*self)
    }
    pub fn get_mut<'a>(&self, game: &'a mut Game) -> Option<&'a mut GameObject> {
        game.game_objects.get_mut(*self)
    }
    pub fn get_coords(&self, game_objects: &IdMap<GameObjectId, GameObject>) -> PixelCoords {
        self.get_coords_safe(game_objects).unwrap()
    }
    pub fn get_coords_safe(
        &self,
        game_objects: &IdMap<GameObjectId, GameObject>,
    ) -> Option<PixelCoords> {
        game_objects.get(*self).map(|x| x.coords)
    }
    pub fn get_coords_game(&self, game: &Game) -> PixelCoords {
        self.get_coords(&game.game_objects)
    }
    pub fn get_coords_game_safe(&self, game: &Game) -> Option<PixelCoords> {
        self.get_coords_safe(&game.game_objects)
    }
    pub fn get_square_coords(&self, game: &Game) -> SquareCoords {
        self.get_coords_game(game).into()
    }
    pub fn get_chunk_coords(&self, game: &Game) -> TerrainChunkCoords {
        self.get_coords_game(game).into()
    }
    pub fn set_hit_box(&self, game: &mut Game, hit_box: HitBox) {
        game.game_objects.get_mut(*self).unwrap().hit_box = hit_box;
    }
    pub fn get_distance_to_point(
        &self,
        game_objects: &IdMap<GameObjectId, GameObject>,
        point: &PixelCoords,
    ) -> f64 {
        let my_coords = self.get_coords(game_objects);
        my_coords.get_distance_to(point)
    }
    pub fn get_distance_to_point_game(&self, game: &Game, point: &PixelCoords) -> f64 {
        let my_coords = self.get_coords_game(game);
        my_coords.get_distance_to(point)
    }
    pub fn get_direction_to_point_minimal(
        &self,
        game_objects: &IdMap<GameObjectId, GameObject>,
        point: &PixelCoords,
    ) -> Angle {
        let my_coords = self.get_coords(game_objects);
        my_coords.get_direction_to(point)
    }
    pub fn get_direction_to_point(&self, game: &Game, point: &PixelCoords) -> Angle {
        self.get_direction_to_point_minimal(&game.game_objects, point)
    }
    pub fn get_distance_to_game(&self, game: &Game, other: &GameObjectId) -> f64 {
        let their_coords = other.get_coords_game(game);
        self.get_distance_to_point_game(game, &their_coords)
    }
    pub fn get_distance_to(
        &self,
        game_objects: &IdMap<GameObjectId, GameObject>,
        other: &GameObjectId,
    ) -> f64 {
        let their_coords = other.get_coords(game_objects);
        self.get_distance_to_point(game_objects, &their_coords)
    }
    pub fn get_direction_to_minimal(
        &self,
        game_objects: &IdMap<GameObjectId, GameObject>,
        other: &GameObjectId,
    ) -> Option<Angle> {
        let my_coords = self.get_coords(game_objects);
        let their_coords = other.get_coords_safe(game_objects);
        their_coords.map(|their_coords| my_coords.get_direction_to(&their_coords))
    }
    pub fn get_direction_to(&self, game: &Game, other: &GameObjectId) -> Option<Angle> {
        self.get_direction_to_minimal(&game.game_objects, other)
    }
    pub fn add_component<T: Component + 'static>(&self, game: &mut Game, component: T) {
        let this = game
            .game_objects
            .get_mut(*self)
            .expect("Can't add component to deleted object");
        this.components
            .components
            .insert(component.get_component_id(), Box::new(component));
    }
    /// does nothing if component not present
    pub fn remove_component(&self, game: &mut Game, component_id: ComponentId) {
        let component = {
            let game_object = game.game_objects.get_mut(*self).unwrap();
            let component = game_object.components.components.remove(component_id);
            if let Some(component) = component {
                component
            } else {
                return;
            }
        };
        component.on_remove(game, *self);
    }
    pub fn set_rotation(&self, game: &mut Game, rotation: Angle) {
        game.movement_system.to_rotate.insert(*self, rotation);
    }
    pub fn get_rotation(&self, game: &mut Game) -> Angle {
        game.game_objects.get(*self).unwrap().rotation
    }
    pub fn remove(&self, game: &mut Game) {
        game.to_delete.push(*self);
    }
    pub fn is_deleted(&self, game_objects: &IdMap<GameObjectId, GameObject>) -> bool {
        game_objects.get(*self).is_none()
    }
}

impl HitBoxed for GameObjectId {
    fn get_hit_box(&self, game: &Game) -> HitBox {
        game.game_objects.get(*self).unwrap().get_hit_box()
    }
}
