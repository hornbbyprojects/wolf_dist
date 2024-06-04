use super::*;

impl Player {
    pub fn update_game_objects(game: &mut Game) {
        let mut updates: Vec<(
            PlayerId,
            Option<GameObjectId>,
            PixelCoords,
            WolfHashSet<GameObjectId>,
            Vec<GameObjectId>,
            Vec<GameObjectId>,
        )> = Vec::new();
        {
            let collision_map =
                CollisionSystem::get_collision_group(game, CollisionGroupId::ClientSideComponent)
                    .map(|collision_group| collision_group.collision_map.borrow());

            for (player_id, player) in game.player_system.players.iter() {
                let view_coords = if let Some(game_object_id) = player.bound_object_id {
                    let view_coords_override = game_object_id.send_get_view_coords_signal(game);
                    if let Some(view_coords_override) = view_coords_override {
                        view_coords_override.extract()
                    } else {
                        if let Some(coords) = game_object_id.get_coords_game_safe(game) {
                            coords
                        } else {
                            player.last_view_coords
                        }
                    }
                } else {
                    player.last_view_coords
                };
                let center_coords: SquareCoords = view_coords.into();
                let nearby_game_objects = match collision_map {
                    Some(ref collision_map) => collision_map.0.get_within_box(
                        center_coords,
                        CLIENT_SIDE_COMPONENT_RENDER_RANGE_SQUARES,
                        CLIENT_SIDE_COMPONENT_RENDER_RANGE_SQUARES,
                    ),
                    None => WolfHashSet::new(),
                };
                let new_game_objects = nearby_game_objects.difference(&player.current_game_objects);
                let removed_game_objects = player
                    .current_game_objects
                    .difference(&nearby_game_objects)
                    .map(|x| *x)
                    .collect();
                let moved_game_objects = player
                    .game_objects_to_update
                    .intersection(&nearby_game_objects);
                let updated_game_objects = new_game_objects
                    .chain(moved_game_objects)
                    .map(|x| *x)
                    .collect();

                updates.push((
                    player_id,
                    player.bound_object_id,
                    view_coords,
                    nearby_game_objects,
                    updated_game_objects,
                    removed_game_objects,
                ));
            }
        }
        for (
            player_id,
            watching_object_id,
            view_coords,
            nearby_game_objects,
            moved_game_objects,
            removed_game_objects,
        ) in updates
        {
            let mut update_components_message = UpdateComponentsMessage {
                updates_by_object: Vec::new(),
            };
            for game_object_id in nearby_game_objects.iter() {
                if let Some(message) = game_object_id.handle_client_side_components(game, player_id)
                {
                    update_components_message.updates_by_object.push(message);
                }
                game.player_system
                    .recently_active_client_side_objects
                    .insert(*game_object_id);
            }
            let view_message = ViewMessage {
                watching_object_id: watching_object_id.map(|x| x.into()),
                view_coords,
            };
            let updated_game_objects = moved_game_objects
                .into_iter()
                .map(|game_object_id| MoveGameObject {
                    game_object_id: game_object_id.into(),
                    coords: game_object_id.get_coords_game(game),
                    rotation: game_object_id.get_rotation(game).into(),
                })
                .collect();
            let deleted_game_objects = removed_game_objects
                .into_iter()
                .map(|game_object_id| RemoveGameObject {
                    game_object_id: game_object_id.into(),
                })
                .collect();
            let update_game_objects_message = UpdateGameObjectsMessage {
                view_message,
                current_tick: game.tick_counter,
                updated_game_objects,
                deleted_game_objects,
            };
            let player = game.player_system.players.get_mut(player_id).unwrap();
            player.last_view_coords = view_coords;
            player.game_objects_to_update = WolfHashSet::new();
            player.current_game_objects = nearby_game_objects;

            player
                .server_messages
                .push(ServerMessage::UpdateGameObjects(
                    update_game_objects_message,
                ));
            player
                .server_messages
                .push(ServerMessage::UpdateComponents(update_components_message));
        }
    }
}
