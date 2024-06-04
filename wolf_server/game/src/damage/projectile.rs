use super::*;
use crate::drawable::BasicDrawingComponent;
use crate::generic::DeleteAtComponent;

// make an object both damaging and mobile, and delete when it fizzles out

#[derive(Clone)]
pub struct Projectile {
    game_object_id: GameObjectId,
    direction: Angle,
    speed: f64,
}

impl Projectile {
    pub fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        direction: Angle,
        speed: f64,
    ) -> ProjectileId {
        let id = game.get_id();
        let projectile = Projectile {
            game_object_id,
            direction,
            speed,
        };
        game.damage_system.projectiles.insert(id, projectile);
        id
    }
    pub fn step(game: &mut Game) {
        let mut to_move = Vec::new();
        for (_id, projectile) in game.damage_system.projectiles.iter() {
            to_move.push(projectile.clone());
        }
        for projectile in to_move {
            projectile.game_object_id.move_direction_game(
                game,
                projectile.direction,
                projectile.speed,
            );
        }
    }
}

#[derive(Clone)]
pub struct ProjectileComponent {
    id: ComponentId,
    projectile_id: ProjectileId,
    damager_id: DamagerId,
}
impl ProjectileComponent {
    pub fn fire_basic_projectile(
        game: &mut Game,
        firer_id: GameObjectId,
        sprite: u32,
        damage: i32,
        hits: Option<u32>,
        starting_coords: PixelCoords,
        direction: Angle,
        speed: f64,
        lifetime: u32,
    ) -> GameObjectId {
        let game_object_id = GameObject::create_game(game, starting_coords);
        let DamagerComponent { damager_id, .. } =
            DamagerComponent::add_to(game, game_object_id, firer_id, hits, damage);
        BasicDrawingComponent::add_to(game, game_object_id, sprite, PROJECTILE_DEPTH);
        ProjectileComponent::add_to(game, game_object_id, damager_id, direction, speed);
        DeleteAtComponent::add_to(game, game_object_id, game.tick_counter + lifetime);
        game_object_id
    }
    pub fn add_to(
        game: &mut Game,
        game_object_id: GameObjectId,
        damager_id: DamagerId,
        direction: Angle,
        speed: f64,
    ) {
        let component_id = game.get_id();
        let projectile_id = Projectile::new(game, game_object_id, direction, speed);
        let comp = ProjectileComponent {
            id: component_id,
            projectile_id,
            damager_id,
        };
        game_object_id.add_damager_fizzled_out_signal_listener(game, comp.clone());
        game_object_id.add_component(game, comp);
    }
}
impl Component for ProjectileComponent {
    fn get_component_id(&self) -> ComponentId {
        self.id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        game.damage_system.projectiles.remove(self.projectile_id);
    }
}
impl DamagerFizzledOutSignalListener for ProjectileComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.id
    }
    fn clone_box(&self) -> Box<dyn DamagerFizzledOutSignalListener> {
        Box::new(self.clone())
    }
    fn receive_damager_fizzled_out_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        damager_id: DamagerId,
    ) {
        if damager_id == self.damager_id {
            owner_id.remove(game);
        }
    }
}
