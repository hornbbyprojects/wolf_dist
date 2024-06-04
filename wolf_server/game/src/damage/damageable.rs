use crate::damage::*;
use std::rc::Rc;
use wolf_interface::CreateComponentData;

define_signal_listener!(Damage, &mut Game, amount: Health);
define_signal_listener!(
    DamageTaken,
    &mut Game,
    amount: Health,
    health_remaining: Health
);

pub struct Damageable {
    pub game_object_id: GameObjectId,
    pub health: Health,
    pub max_health: Health,
}

impl HitBoxed for DamageableId {
    fn get_hit_box(&self, game: &Game) -> HitBox {
        let damageable = game.damage_system.damageables.get(*self).unwrap();
        damageable.game_object_id.get_hit_box(game)
    }
}

impl Damageable {
    pub fn new(game: &mut Game, game_object_id: GameObjectId) -> DamageableId {
        let id = game.get_id();
        let damageable = Damageable {
            game_object_id,
            health: DEFAULT_HEALTH,
            max_health: DEFAULT_HEALTH,
        };
        game.damage_system.damageables.insert(id, damageable);
        id
    }
    pub fn remove(game: &mut Game, id: DamageableId) {
        game.damage_system.damageables.remove(id).unwrap();
    }
}

#[derive(Clone)]
pub struct DamageableComponent {
    component_id: ComponentId,
    pub damageable_id: DamageableId,
    pub client_side_component_id: ClientSideComponentId,
}

impl DamageableComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let damageable_id = Damageable::new(game, owner_id);
        let client_side_component_id = owner_id.add_client_side_component(
            game,
            CreateComponentData::HealthProportionTenThousandths(10000),
        );
        let comp = Rc::new(DamageableComponent {
            component_id,
            damageable_id,
            client_side_component_id,
        });
        owner_id.add_collision_group(game, CollisionGroupId::Damageable);
        owner_id.add_get_healthiness_signal_listener(game, comp.clone());
        owner_id.add_damage_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
    }
}
impl Component for Rc<DamageableComponent> {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_client_side_component(game, self.client_side_component_id);
        owner_id.remove_collision_group(game, CollisionGroupId::Damageable);
        owner_id.remove_get_healthiness_signal_listener(game, self.component_id);
        owner_id.remove_damage_signal_listener(game, self.component_id);
        Damageable::remove(game, self.damageable_id);
    }
}
impl DamageSignalListener for Rc<DamageableComponent> {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn DamageSignalListener> {
        Box::new(Rc::clone(self))
    }
    fn receive_damage_signal(&self, game: &mut Game, owner_id: GameObjectId, amount: Health) {
        let (health, max_health) = {
            let damageable = game
                .damage_system
                .damageables
                .get_mut(self.damageable_id)
                .unwrap();
            damageable.health -= amount;
            (damageable.health.clone(), damageable.max_health.clone())
        };
        let percent = health.0 as f64 / max_health.0 as f64;
        let client_side_component_data =
            CreateComponentData::HealthProportionTenThousandths((percent * 10000.0) as u32);
        owner_id.refresh_client_side_component(
            game,
            self.client_side_component_id,
            client_side_component_data,
        );
        owner_id.send_damage_taken_signal(game, amount, health);
    }
}
impl GetHealthinessSignalListener for Rc<DamageableComponent> {
    fn clone_box(&self) -> Box<dyn GetHealthinessSignalListener> {
        Box::new(Rc::clone(self))
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_healthiness_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
    ) -> MinimumHealthPercent {
        let damageable = game
            .damage_system
            .damageables
            .get(self.damageable_id)
            .unwrap();
        let percent = damageable.health.0 as f64 / damageable.max_health.0 as f64;
        MinimumHealthPercent(percent.max(0.0))
    }
}
