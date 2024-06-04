use super::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(SpellbookWasAbsorbed, &mut Game, absorber: GameObjectId);

pub struct SpellbookAbsorber {
    game_object_id: GameObjectId,
}

impl SpellbookAbsorber {
    pub fn new(game: &mut Game, game_object_id: GameObjectId) -> SpellbookAbsorberId {
        let id = game.get_id();
        let spellbook_absorber = SpellbookAbsorber { game_object_id };
        game.ability_system
            .spellbook_system
            .spellbook_absorbers
            .insert(id, spellbook_absorber);
        id
    }
    pub fn remove(game: &mut Game, id: SpellbookAbsorberId) {
        game.ability_system
            .spellbook_system
            .spellbook_absorbers
            .remove(id);
    }
    pub fn step(game: &mut Game) {
        let mut spellbooks_to_destroy = Vec::new();
        {
            let collision_group =
                match CollisionSystem::get_collision_group(game, CollisionGroupId::Spellbook) {
                    Some(x) => x,
                    None => return,
                };
            let spellbook_collision_map = collision_group.collision_map.borrow();
            for (_id, spellbook_absorber) in game
                .ability_system
                .spellbook_system
                .spellbook_absorbers
                .iter()
            {
                let hitbox = spellbook_absorber.game_object_id.get_hit_box(game);
                let spellbooks = spellbook_collision_map.get_colliding_game(game, hitbox);
                for game_object_id in spellbooks {
                    spellbooks_to_destroy.push((spellbook_absorber.game_object_id, game_object_id));
                }
            }
        }
        for (absorber, id) in spellbooks_to_destroy {
            id.send_spellbook_was_absorbed_signal(game, absorber);
            id.remove(game);
        }
    }
}

pub struct SpellbookAbsorberComponent {
    pub component_id: ComponentId,
    pub spellbook_absorber_id: SpellbookAbsorberId,
}

impl SpellbookAbsorberComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let spellbook_absorber_id = SpellbookAbsorber::new(game, owner_id);
        let comp = SpellbookAbsorberComponent {
            component_id,
            spellbook_absorber_id,
        };
        owner_id.add_component(game, comp);
    }
}

impl Component for SpellbookAbsorberComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        SpellbookAbsorber::remove(game, self.spellbook_absorber_id);
    }
}
