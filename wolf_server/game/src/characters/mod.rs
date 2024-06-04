use crate::combinable::CantCombine;
use crate::game::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(GetCharacterComponentIds, &Game -> CantCombine<ComponentId>);
mod base_character;
pub use base_character::*;
mod lich;
pub use lich::*;
mod knight;
pub use knight::*;

// TED'S WORLD: BASE
mod paladin;
pub use paladin::*;

fn remove_character(game: &mut Game, owner_id: GameObjectId) {
    if let Some(CantCombine(old_character_id)) =
        owner_id.send_get_character_component_ids_signal(game)
    {
        owner_id.remove_component(game, old_character_id);
    }
}
