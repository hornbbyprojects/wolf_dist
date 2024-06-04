use crate::abilities::ability_user::{AbilitiesChangedSignalListener, GetAbilityIconsSignalSender};
use wolf_interface::{ServerMessage, SlotMappingMessage};

use crate::game::*;

/*
This module handles assigning slots to abilities.
This must be)
1) Persistent (if you bind to the same object twice, player should not have to rearrange abilities)
2) Per player (two players may want different bindings)
*/

#[derive(Clone)]
pub struct SlotMappingAssigner {
    component_id: ComponentId,
    player_id: PlayerId,
}

impl Component for SlotMappingAssigner {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_abilities_changed_signal_listener(game, self.component_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl AbilitiesChangedSignalListener for SlotMappingAssigner {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn AbilitiesChangedSignalListener> {
        Box::new(self.clone())
    }
    fn receive_abilities_changed_signal(&self, game: &mut Game, owner_id: GameObjectId) {
        self.update_mapping(game, owner_id);
    }
}

impl SlotMappingAssigner {
    fn update_mapping(&self, game: &mut Game, owner_id: GameObjectId) {
        let ability_ids_with_icons = owner_id
            .send_get_ability_icons_signal(game)
            .map(|x| x.0)
            .unwrap_or(Vec::new());
        let ability_ids: Vec<AbilityId> = ability_ids_with_icons.iter().map(|x| x.0).collect();
        let slot_mapping = game
            .player_system
            .slot_mappings
            .get_mut(&(self.player_id, owner_id))
            .unwrap();
        let mut to_remove = Vec::new();
        for (i, ability_id) in slot_mapping.slot_to_ability_id.iter().enumerate() {
            if let Some(ability_id) = ability_id {
                if !ability_ids.contains(ability_id) {
                    to_remove.push(i);
                }
            }
        }
        for i in to_remove {
            slot_mapping.slot_to_ability_id[i] = None;
        }
        let mut to_add = Vec::new();
        // This could be optimised, but I don't expect it to be a problem, because slots and number
        // of abilities available should always be small
        for ability_id in ability_ids {
            if !slot_mapping.slot_to_ability_id.contains(&Some(ability_id)) {
                to_add.push(ability_id);
            }
        }
        for id in to_add {
            let mut no_free_slots = true;
            for i in 0..slot_mapping.slot_to_ability_id.len() {
                if slot_mapping.slot_to_ability_id[i].is_none() {
                    no_free_slots = false;
                    slot_mapping.slot_to_ability_id[i] = Some(id);
                    break;
                }
            }
            if no_free_slots {
                break;
            }
        }
        let slot_to_ability_icon = slot_mapping
            .slot_to_ability_id
            .iter()
            .map(|ability_id_opt| {
                /* O(N^2), but ability counts should be small */
                if let Some(ability_id) = ability_id_opt {
                    let icon = ability_ids_with_icons
                        .iter()
                        .find(|(other_id, icon)| other_id == ability_id)
                        .unwrap()
                        .1;
                    Some(icon)
                } else {
                    None
                }
            })
            .collect();
        let message = SlotMappingMessage {
            slot_to_ability_icon,
        };
        game.player_system
            .players
            .get_mut(self.player_id)
            .unwrap()
            .server_messages
            .push(ServerMessage::SlotMapping(message));
    }
    pub fn add_to(game: &mut Game, owner: GameObjectId, player_id: PlayerId) -> Self {
        let component_id = game.get_id();
        assert!(
            game.player_system
                .slot_mappings
                .insert((player_id, owner), SlotMapping::new())
                .is_none(),
            "Assigned two slot mappings for the same player to the same game_object_id!"
        );
        let comp = SlotMappingAssigner {
            component_id,
            player_id,
        };
        owner.add_component(game, comp.clone());
        owner.add_abilities_changed_signal_listener(game, comp.clone());
        comp.update_mapping(game, owner);
        comp
    }
}

pub struct SlotMapping {
    pub slot_to_ability_id: Vec<Option<AbilityId>>,
}

impl SlotMapping {
    pub fn new() -> Self {
        let mut slot_to_ability_id = Vec::new();
        for _ in 0..8 {
            slot_to_ability_id.push(None);
        }
        SlotMapping { slot_to_ability_id }
    }
}
