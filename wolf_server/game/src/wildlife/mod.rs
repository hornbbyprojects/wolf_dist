use crate::{
    damage::{DamageableComponent, DeleteOnDeathComponent, DieOnNoHealthComponent},
    game::*,
    hunting::PreyComponent,
    resources::{ResourceAmount, ResourceDropperComponent, Resources},
};
use rand::thread_rng;
use std::f64::consts::PI;
use wolf_hash_map::WolfHashMap;

const HOPPER_CREATURE_SPEED: f64 = 4.0;
const GRAZE_DISTANCE: f64 = 300.0;
const GRAZE_CLOSE_ENOUGH: f64 = 50.0;
const GRAZE_REST_TIME: u32 = 250;

pub struct WildlifeSystem {
    wandering_herbivores: IdMap<WanderingHerbivoreId, WanderingHerbivore>,
}

impl WildlifeSystem {
    pub fn new() -> Self {
        WildlifeSystem {
            wandering_herbivores: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        WanderingHerbivore::step(game);
    }
}

struct WanderingHerbivore {
    game_object_id: GameObjectId,
    target_square: Option<SquareCoords>,
    resting_for: u32,
}

impl WanderingHerbivore {
    fn step(game: &mut Game) {
        for (_id, wandering_herbivore) in game.wildlife_system.wandering_herbivores.iter_mut() {
            let pixel_coords = wandering_herbivore
                .game_object_id
                .get_coords(&game.game_objects);
            match wandering_herbivore.target_square {
                Some(square_coords) => {
                    let distance = pixel_coords.get_distance_to(&square_coords.center_pixel());
                    if distance < GRAZE_CLOSE_ENOUGH {
                        wandering_herbivore.target_square = None;
                        wandering_herbivore.resting_for = GRAZE_REST_TIME;
                        wandering_herbivore
                            .game_object_id
                            .intend_stop(&mut game.movement_system.intend_move_system)
                    } else {
                        wandering_herbivore.game_object_id.intend_move_to_point(
                            &mut game.movement_system.intend_move_system,
                            square_coords.center_pixel(),
                        )
                    }
                }
                None => {
                    if wandering_herbivore.resting_for > 0 {
                        wandering_herbivore.resting_for -= 1;
                    } else {
                        let direction = thread_rng().gen_range(0.0..PI * 2.0);
                        let target_coords = pixel_coords
                            .offset_direction(Angle::enforce_range(direction), GRAZE_DISTANCE);
                        wandering_herbivore.target_square = Some(target_coords.into());
                    }
                }
            }
        }
    }
    fn create(game: &mut Game, game_object_id: GameObjectId) -> WanderingHerbivoreId {
        let id = game.get_id();
        let wandering_herbivore = WanderingHerbivore {
            game_object_id,
            target_square: None,
            resting_for: 0,
        };
        game.wildlife_system
            .wandering_herbivores
            .insert(id, wandering_herbivore);
        id
    }
    fn remove(game: &mut Game, id: WanderingHerbivoreId) {
        game.wildlife_system.wandering_herbivores.remove(id);
    }
}

pub struct WanderingHerbivoreComponent {
    component_id: ComponentId,
    wandering_herbivore_id: WanderingHerbivoreId,
}
impl Component for WanderingHerbivoreComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        WanderingHerbivore::remove(game, self.wandering_herbivore_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
impl WanderingHerbivoreComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let wandering_herbivore_id = WanderingHerbivore::create(game, owner_id);
        let comp = WanderingHerbivoreComponent {
            component_id,
            wandering_herbivore_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}

pub fn create_hopper_creature(game: &mut Game, coords: PixelCoords) {
    let game_object_id = GameObject::create_game(game, coords);
    HopperComponent::add_to(game, game_object_id, HOPPER_CREATURE_SPEED);
    WanderingHerbivoreComponent::add_to(game, game_object_id);
    let mut sprites = WolfHashMap::new();
    sprites.insert(CardinalDirection::Left, CREATURE_SPRITE_LEFT);
    sprites.insert(CardinalDirection::Right, CREATURE_SPRITE_RIGHT);
    sprites.insert(CardinalDirection::Up, CREATURE_SPRITE_UP);
    sprites.insert(CardinalDirection::Down, CREATURE_SPRITE_DOWN);
    FacingSpriteComponent::add_to(game, game_object_id, sprites, DEFAULT_DEPTH);
    DamageableComponent::add_to(game, game_object_id);
    PreyComponent::add_to(game, game_object_id);
    DieOnNoHealthComponent::add_to(game, game_object_id);
    DeleteOnDeathComponent::add_to(game, game_object_id);
    ResourceDropperComponent::add_to(game, game_object_id, Resources::food(ResourceAmount(20)));
    add_health_bar(game, game_object_id);
}
