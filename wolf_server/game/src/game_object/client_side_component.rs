use super::*;
use crate::game::*;
use wolf_interface::*;

#[derive(Debug, Clone)]
pub enum ClientSideComponentVisibility {
    All,
    BoundPlayerOnly,
}
pub struct ClientSideComponent {
    pub data: CreateComponentData,
    // None if not updated last tick
    update: Option<UpdateComponentData>,
    visibility: ClientSideComponentVisibility,
}

impl GameObjectId {
    pub fn add_client_side_component_with_visibility(
        &self,
        game: &mut Game,
        data: CreateComponentData,
        visibility: ClientSideComponentVisibility,
    ) -> ClientSideComponentId {
        let client_side_component_id = game.get_id();
        let game_object = game.game_objects.get_mut(*self).unwrap();
        let client_side_component = ClientSideComponent {
            data,
            update: None,
            visibility,
        };
        game_object
            .client_side_components
            .insert(client_side_component_id, client_side_component);
        self.add_collision_group(game, CollisionGroupId::ClientSideComponent);
        client_side_component_id
    }
    pub fn add_client_side_component(
        &self,
        game: &mut Game,
        data: CreateComponentData,
    ) -> ClientSideComponentId {
        self.add_client_side_component_with_visibility(
            game,
            data,
            ClientSideComponentVisibility::All,
        )
    }

    pub fn refresh_client_side_component(
        &self,
        game: &mut Game,
        client_side_component_id: ClientSideComponentId,
        data: CreateComponentData,
    ) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        let client_side_component = ClientSideComponent {
            data,
            update: None,
            visibility: ClientSideComponentVisibility::All,
        };
        let already_had_component = game_object
            .client_side_components
            .insert(client_side_component_id, client_side_component)
            .is_some();
        assert!(already_had_component);
        game_object
            .last_sent_client_side_components
            .remove(&client_side_component_id);
    }

    pub fn remove_client_side_component(
        &self,
        game: &mut Game,
        client_side_component_id: ClientSideComponentId,
    ) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        game_object
            .client_side_components
            .remove(client_side_component_id);
        self.remove_collision_group(game, CollisionGroupId::ClientSideComponent);
    }

    /// returns None if there are no updates to send
    pub fn handle_client_side_components(
        &self,
        game: &mut Game,
        player_id: PlayerId,
    ) -> Option<UpdateComponentsForObjectMessage> {
        let is_bound_object = {
            let player = game.player_system.players.get(player_id).unwrap();
            player.bound_object_id == Some(*self)
        };
        let game_object = game.game_objects.get_mut(*self).unwrap();

        let mut create_messages = Vec::new();
        let mut update_messages = Vec::new();
        let mut remove_messages = Vec::new();

        let mut last_sent = game_object.last_sent_client_side_components.clone();

        for (client_side_component_id, client_side_component) in
            game_object.client_side_components.iter()
        {
            match client_side_component.visibility {
                ClientSideComponentVisibility::All => {}
                ClientSideComponentVisibility::BoundPlayerOnly => {
                    if !is_bound_object {
                        continue;
                    }
                }
            }
            let sent_last_tick = last_sent.remove(&client_side_component_id);
            let sent_to_player_last_tick =
                sent_last_tick && game_object.players_sent_to_last_tick.contains(&player_id);
            if sent_to_player_last_tick {
                if let Some(ref update_data) = client_side_component.update {
                    let _update_message = UpdateComponentMessage {
                        component_id: client_side_component_id.into(),
                        data: update_data.clone(),
                    };
                    update_messages.push(_update_message);
                }
            } else {
                let create_message = CreateComponentMessage {
                    component_id: client_side_component_id.into(),
                    data: client_side_component.data.clone(),
                };
                create_messages.push(create_message);
            }
        }
        for remaining_id in last_sent.into_iter() {
            remove_messages.push(RemoveComponentMessage {
                component_id: remaining_id.into(),
            });
        }

        game_object.players_sent_to_this_tick.insert(player_id);

        if create_messages.is_empty() && update_messages.is_empty() && remove_messages.is_empty() {
            None
        } else {
            let colour_creates: Vec<CreateComponentMessage> = create_messages
                .clone()
                .into_iter()
                .filter(|msg| match msg.data {
                    CreateComponentData::Coloured(_) => true,
                    _ => false,
                })
                .collect();
            Some(UpdateComponentsForObjectMessage {
                game_object_id: (*self).into(),
                created_components: create_messages,
                updated_components: update_messages,
                removed_components: remove_messages,
            })
        }
    }
}
