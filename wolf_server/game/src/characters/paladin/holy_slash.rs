use wolf_hash_map::WolfHashSet;
use wolf_interface::SlashAnimationData;

use crate::basic_client_side_component::BasicClientSideComponent;

use super::*;

const SLASH_EVERY: u32 = 50;
const BEAM_EVERY: u32 = 5;
const SLASH_OFFSET: f64 = 30.0;
const SLASH_LIFETIME: u32 = 50;
const NUMBER_OF_BEAMS: u32 = 3;
const BEAM_RANGE: f64 = 500.0;

pub struct HolySlashAbility {
    ability_id: AbilityId,
    active_slashes: WolfHashSet<GameObjectId>,
    next_slash: u32,
    next_beam: u32,
    beams_left: u32,
}

impl HolySlashAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        HolySlashAbility {
            ability_id,
            active_slashes: WolfHashSet::new(),
            next_slash: 0,
            next_beam: 0,
            beams_left: 0,
        }
    }
}

const BEAM_LENGTH: f64 = 75.0;
const BEAM_BREADTH: f64 = 5.0;
const BEAM_TIME_TO_PROPOGATE: u32 = 2;
const BEAM_LIFETIME: u32 = 8;
fn create_beam(game: &mut Game, firer_id: GameObjectId, start: PixelCoords, end: PixelCoords) {
    let direction = start.get_direction_to(&end);
    let num_beams: u32 = (BEAM_RANGE / BEAM_LENGTH).ceil() as u32;
    for i in 0..num_beams {
        TimerSystem::add_timer(
            game,
            Box::new(move |game| {
                let offset = BEAM_LENGTH * (i as f64 + 0.5);
                let pos = start.offset_direction(direction, offset);
                let hit_box = HitBox::new_at_zero(BEAM_LENGTH as i32 / 2, BEAM_BREADTH as i32 / 2);
                let beam = GameObject::create_with_hit_box(game, pos, hit_box);
                beam.set_rotation(game, direction);
                BasicDrawingComponent::add_to(game, beam, HOLY_BEAM_SPRITE, PROJECTILE_DEPTH);
                DamagerComponent::add_to(game, beam, firer_id, None, DEFAULT_HEALTH.0 / 10);
                TimerSystem::add_timer(
                    game,
                    Box::new(move |game| beam.remove(game)),
                    BEAM_LIFETIME,
                );
            }),
            i * BEAM_TIME_TO_PROPOGATE,
        );
    }
}
impl Ability for HolySlashAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn get_ability_icon(&self) -> u32 {
        HOLY_SLASH_SPRITE
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        if self.next_slash <= game.tick_counter {
            self.beams_left = NUMBER_OF_BEAMS;
            self.next_slash = game.tick_counter + SLASH_EVERY;
            let caster_coords = caster.get_coords(&game.game_objects);
            let direction = caster_coords.get_direction_to(&target_coords);
            let slash_coords = caster_coords.offset_direction(direction, SLASH_OFFSET);
            let hit_box = HitBox::new_at_zero(18, 75);
            let slash = GameObject::create_with_hit_box(game, slash_coords, hit_box);
            slash.set_rotation(game, direction);
            BasicClientSideComponent::add_to(
                game,
                slash,
                CreateComponentData::SlashAnimation(SlashAnimationData {
                    start_tick: game.tick_counter,
                }),
            );
            self.active_slashes.insert(slash);
            TimerSystem::add_timer(
                game,
                Box::new(move |game| slash.remove(game)),
                SLASH_LIFETIME,
            );
        }
        if self.beams_left > 0 && self.next_beam <= game.tick_counter {
            self.beams_left -= 1;
            self.next_beam = game.tick_counter + BEAM_EVERY;
            let mut to_remove = Vec::new();
            let mut shoot_from = Vec::new();
            for active_slash in self.active_slashes.iter() {
                if let Some(coords) = active_slash.get_coords_safe(&game.game_objects) {
                    shoot_from.push(coords);
                } else {
                    to_remove.push(*active_slash);
                }
            }
            for id in to_remove {
                self.active_slashes.remove(&id);
            }
            for coords in shoot_from {
                create_beam(game, caster, coords, target_coords);
            }
        }
    }
}
