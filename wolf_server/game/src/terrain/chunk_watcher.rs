use crate::game::*;
use crate::player::BoundPlayerSignalListener;
use crate::terrain::LOAD_CHUNKS_WITHIN;
use wolf_hash_map::WolfHashSet;
use wolf_interface::{ChunkUnloadMessage, ChunkUpdateMessage, ServerMessage};

use super::get_chunk_index_from_relative_coords;

/// Sends messages to a player to keep them up to date on terrain
pub struct ChunkWatcher {
    player_id: Option<PlayerId>,
    game_object_id: GameObjectId,
    currently_loaded: WolfHashSet<TerrainChunkCoords>,
}

impl ChunkWatcher {
    fn new(game: &mut Game, game_object_id: GameObjectId) -> ChunkWatcherId {
        let id = game.get_id();
        let chunk_watcher = ChunkWatcher {
            player_id: None,
            game_object_id,
            currently_loaded: WolfHashSet::new(),
        };
        game.terrain.chunk_watchers.insert(id, chunk_watcher);
        id
    }
    fn remove(game: &mut Game, id: ChunkWatcherId) {
        game.terrain.chunk_watchers.remove(id);
    }
    pub fn step(game: &mut Game) {
        let mut to_send = Vec::new();
        for (id, chunk_watcher) in game.terrain.chunk_watchers.iter() {
            let player_id = match chunk_watcher.player_id {
                Some(x) => x,
                None => continue,
            };
            let watching = crate::time_system!(square_of_coords_centered(
                chunk_watcher.game_object_id.get_chunk_coords(game),
                LOAD_CHUNKS_WITHIN,
            ));
            let mut update_messages = Vec::new();
            let mut to_unload = Vec::new();
            let mut chunks_redrawn_debug = 0;
            let mut squares_redrawn_debug = 0;
            for coords in chunk_watcher.currently_loaded.iter() {
                chunks_redrawn_debug += 1;
                if let Some(chunk) = game.terrain.chunks.get(coords) {
                    let square_updates = crate::time_system!(chunk
                        .squares_to_redraw
                        .iter()
                        .map(|relative_coords| {
                            squares_redrawn_debug += 1;
                            (
                                *relative_coords,
                                chunk.chunk_squares
                                    [get_chunk_index_from_relative_coords(*relative_coords)]
                                .to_sprites_vec(),
                            )
                        })
                        .collect());
                    let message = ChunkUpdateMessage {
                        coords: *coords,
                        square_updates,
                    };
                    update_messages.push(ServerMessage::ChunkUpdate(message));
                } else {
                    to_unload.push(*coords);
                }
            }
            #[cfg(feature = "timing")]
            if game.tick_counter % TIME_EVERY == 0 {
                println!(
                    "Redrawing {} squares for {} chunks",
                    squares_redrawn_debug, chunks_redrawn_debug
                );
            }
            let needs_sending: Vec<TerrainChunkCoords> = crate::time_system!(watching
                .difference(&chunk_watcher.currently_loaded)
                .map(|x| x.clone())
                .collect());
            to_send.push((id, player_id, needs_sending, update_messages, to_unload))
        }
        for (chunk_watcher_id, player_id, chunk_coords_to_send, update_messages, to_unload) in
            to_send
        {
            {
                let chunk_watcher = game
                    .terrain
                    .chunk_watchers
                    .get_mut(chunk_watcher_id)
                    .unwrap();
                crate::time_system!(chunk_watcher
                    .currently_loaded
                    .extend(chunk_coords_to_send.clone().into_iter()));
                crate::time_system!(for coords in to_unload.iter() {
                    chunk_watcher.currently_loaded.remove(coords);
                });
            }
            let unload_message = if to_unload.is_empty() {
                None
            } else {
                Some(ServerMessage::ChunkUnload(ChunkUnloadMessage {
                    coords: to_unload,
                }))
            };
            let mut messages: Vec<ServerMessage> = crate::time_system!(chunk_coords_to_send
                .into_iter()
                .filter_map(|coords| {
                    game.terrain
                        .chunks
                        .get(&coords)
                        .map(|chunk| ServerMessage::ChunkInfo(chunk.get_info_message(coords)))
                })
                .chain(update_messages.into_iter())
                .chain(unload_message.into_iter())
                .collect());
            let player = game.player_system.players.get_mut(player_id).unwrap();
            crate::time_system!(player.server_messages.append(&mut messages));
        }
        for (_coords, chunk) in game.terrain.chunks.iter_mut() {
            chunk.squares_to_redraw = Vec::new();
        }
    }
}
#[derive(Clone)]
pub struct ChunkWatcherComponent {
    component_id: ComponentId,
    chunk_watcher_id: ChunkWatcherId,
}

impl ChunkWatcherComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let chunk_watcher_id = ChunkWatcher::new(game, owner_id);
        let component_id = game.get_id();
        let comp = ChunkWatcherComponent {
            component_id,
            chunk_watcher_id,
        };
        owner_id.add_bound_player_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
    }
}
impl Component for ChunkWatcherComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        ChunkWatcher::remove(game, self.chunk_watcher_id);
        owner.remove_bound_player_signal_listener(game, self.component_id);
    }
}
impl BoundPlayerSignalListener for ChunkWatcherComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn BoundPlayerSignalListener> {
        Box::new(self.clone())
    }
    fn receive_bound_player_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        player_id: PlayerId,
    ) {
        let chunk_watcher = game
            .terrain
            .chunk_watchers
            .get_mut(self.chunk_watcher_id)
            .unwrap();
        chunk_watcher.player_id = Some(player_id);
        chunk_watcher.currently_loaded = WolfHashSet::new();
    }
}
