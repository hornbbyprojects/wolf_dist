use crate::game::*;
use signal_listener_macro::define_signal_listener;

mod damageable;
pub use damageable::*;
mod damager;
pub use damager::*;
mod projectile;
pub use projectile::*;
mod melee;
pub use melee::*;
mod death;
pub use death::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Health(pub i32);
pub const DEFAULT_HEALTH: Health = Health(10000);
pub const SMALL_DAMAGE: i32 = 1000;
pub const MEDIUM_DAMAGE: i32 = 3000;
pub const LARGE_DAMAGE: i32 = 8000;

pub struct MinimumHealthPercent(pub f64);
impl MinimumHealthPercent {
    fn combine_result(self, other: Self) -> Self {
        MinimumHealthPercent(self.0.min(other.0))
    }
}
define_signal_listener!(GetHealthiness, &Game -> MinimumHealthPercent);

pub struct DamageSystem {
    pub damageables: IdMap<DamageableId, Damageable>,
    pub damagers: IdMap<DamagerId, Damager>,

    pub projectiles: IdMap<ProjectileId, Projectile>,
}
impl DamageSystem {
    pub fn new() -> DamageSystem {
        DamageSystem {
            damageables: IdMap::new(),
            damagers: IdMap::new(),

            projectiles: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        Damager::step(game);
    }
}

impl std::ops::Add for Health {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Health(self.0 + other.0)
    }
}
impl std::ops::AddAssign for Health {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}
impl std::ops::Sub for Health {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Health(self.0 - other.0)
    }
}
impl std::ops::SubAssign for Health {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}
