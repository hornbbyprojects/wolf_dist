use crate::{
    combinable::OrBool,
    game::*,
    monsters::Charger,
    notifications::{
        DESERT_QUEST_COMPLETE_NOTIFICATION_ID, DESERT_QUEST_DISTANCE_NOTIFICATION_ID,
        DESERT_QUEST_START_NOTIFICATION_ID,
    },
    wildlife::WanderingHerbivoreComponent,
};
use std::f64::consts::PI;

use coords::Angle;
use id::IdMap;
use wolf_hash_map::WolfHashSet;

const SPAWN_CHARGERS_Y: PixelNum = PixelNum::const_from_int(0);
const SPAWN_CHARGERS_STARTING_DISTANCE: PixelNum = PixelNum::const_from_int(200);
const CROSS_DESERT_QUEST_VICTORY_Y: PixelNum = PixelNum::const_from_int(6000);

pub struct QuestSystem {
    cross_desert_quests: IdMap<GameObjectId, CrossDesertQuest>,
    walking_upwards: WolfHashSet<GameObjectId>,
}

impl QuestSystem {
    pub fn new() -> Self {
        QuestSystem {
            cross_desert_quests: IdMap::new(),
            walking_upwards: WolfHashSet::new(),
        }
    }
}

impl QuestSystem {
    pub fn step(game: &mut Game) {
        for id in game.quest_system.walking_upwards.iter() {
            id.intend_move_in_direction_minimal(
                &mut game.movement_system.intend_move_system,
                Angle::enforce_range(PI / 2.0),
            );
        }
        CrossDesertQuest::step(game);
    }
}
#[derive(Clone, Debug)]
struct PostQuestInvincibleComponent {
    component_id: ComponentId,
}
impl AboutToBeDamagedSignalListener for PostQuestInvincibleComponent {
    fn receive_about_to_be_damaged_signal(
        &self,
        game: &mut Game,
        owner: GameObjectId,
        damager_id: GameObjectId,
        firer_id: GameObjectId,
    ) -> OrBool {
        OrBool(true)
    }

    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }

    fn clone_box(&self) -> Box<dyn AboutToBeDamagedSignalListener> {
        Box::new(self.clone())
    }
}
impl PostQuestInvincibleComponent {
    fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let comp = PostQuestInvincibleComponent { component_id };
        owner_id.add_about_to_be_damaged_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}
impl Component for PostQuestInvincibleComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_about_to_be_damaged_signal_listener(game, self.component_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
pub struct CrossDesertQuest {
    pub spawned_chargers: bool,
    pub y_started_at: PixelNum,
}

impl CrossDesertQuest {
    fn step(game: &mut Game) {
        let mut quests_to_end = Vec::new();
        let mut to_make_invincible = Vec::new();
        let mut chargers_to_spawn = Vec::new();
        for (id, quest) in game.quest_system.cross_desert_quests.iter_mut() {
            if let Some(coords) = id.get_coords_safe(&game.game_objects) {
                let distance_travelled = coords.get_y() - quest.y_started_at;
                if !quest.spawned_chargers && distance_travelled > SPAWN_CHARGERS_Y {
                    chargers_to_spawn.push(id);
                    quest.spawned_chargers = true;
                }
                if distance_travelled > CROSS_DESERT_QUEST_VICTORY_Y {
                    // Victory!
                    id.send_notification(
                        game.tick_counter,
                        &mut game.player_system,
                        DESERT_QUEST_COMPLETE_NOTIFICATION_ID,
                        "You have completed the quest!".to_string(),
                    );
                    id.clear_notification(
                        &mut game.player_system,
                        DESERT_QUEST_DISTANCE_NOTIFICATION_ID,
                    );
                    id.clear_notification(
                        &mut game.player_system,
                        DESERT_QUEST_START_NOTIFICATION_ID,
                    );
                    if let Some(player_id) = game
                        .player_system
                        .players_by_game_object
                        .get(id)
                        .map(|x| *x)
                    {
                        player_id.unbind(&mut game.player_system);
                    }
                    game.quest_system.walking_upwards.insert(id);
                    to_make_invincible.push(id);
                    quests_to_end.push(id);
                } else {
                    let distance_message = format!(
                        "You have covered {} of {} metres",
                        distance_travelled.to_num::<f64>(),
                        CROSS_DESERT_QUEST_VICTORY_Y.to_num::<f64>(),
                    );
                    id.send_notification(
                        game.tick_counter,
                        &mut game.player_system,
                        DESERT_QUEST_DISTANCE_NOTIFICATION_ID,
                        distance_message,
                    );
                }
            } else {
            }
        }
        for id in chargers_to_spawn {
            let coords = id.get_coords_game(&game);
            let target_coords =
                coords.translate_fixed(PixelNum::from_num(0), -SPAWN_CHARGERS_STARTING_DISTANCE);
            for i in 0..10 {
                let direction = Angle::enforce_range(rand::thread_rng().gen_range(0.0..2.0 * PI));
                let spawn_coords = target_coords.offset_direction(direction, 50.0);
                Charger::new(game, spawn_coords, id);
            }
        }
        for id in to_make_invincible {
            PostQuestInvincibleComponent::add_to(game, id);
        }
        for id in quests_to_end {
            game.quest_system.cross_desert_quests.remove(id);
        }
    }
    pub fn begin(game: &mut Game, id: GameObjectId) {
        let y_started_at = id.get_coords(&game.game_objects).get_y();
        id.send_notification(
            game.tick_counter,
            &mut game.player_system,
            DESERT_QUEST_START_NOTIFICATION_ID,
            "You must travel to the north!".to_string(),
        );
        game.quest_system.cross_desert_quests.insert(
            id,
            CrossDesertQuest {
                spawned_chargers: false,
                y_started_at,
            },
        );
    }
}

pub fn spawn_quest_guide(game: &mut Game) {
    let id = GameObject::create_game(game, PixelCoords::new_at_zero());
    BasicDrawingComponent::add_to(game, id, VILLAGER_SPRITE, DEFAULT_DEPTH);
    WanderingHerbivoreComponent::add_to(game, id);
    WalkerComponent::add_to(game, id, 2.0, 1.0);
    id.speak_safe(game, "To start your quest, cast the questor spell!", None);
}
