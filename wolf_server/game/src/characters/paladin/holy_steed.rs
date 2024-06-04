use super::*;

pub struct HolySteedAbility {
    ability_id: AbilityId,
    active_steed: Option<GameObjectId>,
}

impl HolySteedAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        HolySteedAbility {
            ability_id,
            active_steed: None,
        }
    }
}

impl Ability for HolySteedAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn get_ability_icon(&self) -> u32 {
        SPELLBOOK_SPRITE
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        if let Some(active_steed) = self.active_steed {
            if !active_steed.is_deleted(&game.game_objects) {
                active_steed.remove(game);
                self.active_steed = None;
                return;
            }
        }
        let mounted_id = GameObject::create_game(game, target_coords);
        let mut sprites = WolfHashMap::new();
        sprites.insert(CardinalDirection::Left, DRAGON_SPRITE_LEFT);
        sprites.insert(CardinalDirection::Right, DRAGON_SPRITE_RIGHT);
        sprites.insert(CardinalDirection::Up, DRAGON_SPRITE_UP);
        sprites.insert(CardinalDirection::Down, DRAGON_SPRITE_DOWN);
        FacingSpriteComponent::add_to(game, mounted_id, sprites, CORPSE_DEPTH);
        WalkerComponent::add_to(game, mounted_id, 20.0, 3.0);
        MounterComponent::add_to(game, caster, mounted_id);
        self.active_steed = Some(mounted_id);
    }
}
