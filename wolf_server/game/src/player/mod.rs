use crate::abilities::SlotMapping;
use crate::abilities::SlotMappingAssigner;
use crate::combinable::CantCombine;
use crate::game::*;
use crate::time_system;
use signal_listener_macro::define_signal_listener;
use wolf_hash_map::WolfHashMap;
use wolf_hash_map::WolfHashSet;
use wolf_interface::*;

mod command_processing;
pub use command_processing::*;
pub mod notifications;
mod update;

pub const CLIENT_SIDE_COMPONENT_RENDER_RANGE_SQUARES: i64 = 20;

define_signal_listener!(BoundPlayer, &mut Game, player_id: PlayerId);
define_signal_listener!(GetViewCoords, &Game -> CantCombine<PixelCoords>);

pub struct PlayerSystem {
    pub players: IdMap<PlayerId, Player>,
    pub slot_mappings: WolfHashMap<(PlayerId, GameObjectId), SlotMapping>,
    pub recently_active_client_side_objects: WolfHashSet<GameObjectId>,
    pub players_by_game_object: IdMap<GameObjectId, PlayerId>,
}

pub struct Player {
    pub commands: Vec<Command>,
    pub server_messages: Vec<ServerMessage>,
    pub current_game_objects: WolfHashSet<GameObjectId>,
    pub game_objects_to_update: WolfHashSet<GameObjectId>,

    pub bound_object_id: Option<GameObjectId>,
    pub last_view_coords: PixelCoords,
    pub notifications: IdMap<NotificationId, Notification>,
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
            players: IdMap::new(),
            slot_mappings: WolfHashMap::new(),
            // Used to determine which components should be sent as updates
            recently_active_client_side_objects: WolfHashSet::new(),
            players_by_game_object: IdMap::new(),
        }
    }
    pub fn end_step(game: &mut Game) {
        time_system!(Player::end_step(game));
        time_system!(Self::update_active_client_side_objects(game));
    }
    pub fn update_active_client_side_objects(game: &mut Game) {
        let mut new_active_client_side_objects = WolfHashSet::new();
        for game_object_id in game
            .player_system
            .recently_active_client_side_objects
            .iter()
        {
            if let Some(game_object) = game.game_objects.get_mut(*game_object_id) {
                game_object.players_sent_to_last_tick = std::mem::replace(
                    &mut game_object.players_sent_to_this_tick,
                    WolfHashSet::new(),
                );
                if game_object.players_sent_to_last_tick.is_empty() {
                    game_object.last_sent_client_side_components = WolfHashSet::new();
                } else {
                    game_object.last_sent_client_side_components = game_object
                        .client_side_components
                        .iter()
                        .map(|(k, _v)| k)
                        .collect();
                    new_active_client_side_objects.insert(*game_object_id);
                }
            }
        }
        game.player_system.recently_active_client_side_objects = new_active_client_side_objects;
    }
}

impl Player {
    pub fn create(game: &mut Game) -> PlayerId {
        let player = Player {
            commands: Vec::new(),
            server_messages: Vec::new(),
            current_game_objects: WolfHashSet::new(),
            game_objects_to_update: WolfHashSet::new(),
            bound_object_id: None,
            last_view_coords: PixelCoords::new_at_zero(),
            notifications: IdMap::new(),
        };
        let player_id = game.get_id();
        game.player_system.players.insert(player_id, player);
        let game_object_id = game.create_basic_body();
        player_id.bind_to_object(game, game_object_id);
        player_id
    }
    pub fn end_step(game: &mut Game) {
        time_system!(Player::handle_commands(game));
        time_system!(Player::update_game_objects(game));
    }
    pub fn send_server_message_in_range(
        game: &mut Game,
        message: ServerMessage,
        center_coords: PixelCoords,
        range: f64,
    ) {
        for (_id, player) in game.player_system.players.iter_mut() {
            if let Some(game_object_id) = player.bound_object_id {
                let game_object = game.game_objects.get(game_object_id).unwrap();
                let distance = center_coords.get_distance_to(&game_object.coords);
                if distance > range {
                    continue;
                }
                player.server_messages.push(message.clone());
            }
        }
    }
}
impl PlayerId {
    pub fn bind_to_object(&self, game: &mut Game, game_object_id: GameObjectId) {
        if game
            .player_system
            .slot_mappings
            .get(&(*self, game_object_id))
            .is_none()
        {
            SlotMappingAssigner::add_to(game, game_object_id, *self);
        }
        game_object_id.send_bound_player_signal(game, *self);
        game.player_system
            .players
            .get_mut(*self)
            .unwrap()
            .bound_object_id = Some(game_object_id);
        game.player_system
            .players_by_game_object
            .insert(game_object_id, *self);
    }
    pub fn unbind(&self, player_system: &mut PlayerSystem) {
        let mut player = player_system.players.get_mut(*self).unwrap();
        let bound_object = player.bound_object_id.take();
        if let Some(bound_object) = bound_object {
            player_system.players_by_game_object.remove(bound_object);
        }
    }
}
