//"spellbook"s perform arbitrary effects on those who pick them up
use crate::drawable::BasicDrawingComponent;
use crate::game::*;

mod spellbook;
pub use spellbook::*;

mod spellbook_absorber;
pub use spellbook_absorber::*;

mod lich_book;
pub use lich_book::*;

pub struct SpellbookSystem {
    pub spellbook_absorbers: IdMap<SpellbookAbsorberId, SpellbookAbsorber>,
}

impl SpellbookSystem {
    pub fn new() -> SpellbookSystem {
        SpellbookSystem {
            spellbook_absorbers: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        SpellbookAbsorber::step(game);
    }
    pub fn create_spellbook(game: &mut Game, coords: PixelCoords) {
        let game_object_id = GameObject::create_game(game, coords);
        SpellbookComponent::add_to(game, game_object_id);
        BasicDrawingComponent::add_to(game, game_object_id, SPELLBOOK_SPRITE, PROJECTILE_DEPTH);
        LichBookComponent::add_to(game, game_object_id);
    }
}
