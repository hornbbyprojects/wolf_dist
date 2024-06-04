use crate::game::*;

const SPEECH_TIME: u32 = 100;

pub struct Speech {
    say_until: Option<u32>,
    client_side_component_id: ClientSideComponentId,
}
impl Speech {
    pub fn step(game: &mut Game) {
        let mut to_remove = Vec::new();
        for (id, speech) in game.speeches.iter() {
            if let Some(say_until) = speech.say_until {
                if say_until > game.tick_counter {
                    to_remove.push((id, speech.client_side_component_id));
                }
            }
        }
        for (game_object_id, csc_id) in to_remove {
            if !game_object_id.is_deleted(&game.game_objects) {
                game_object_id.remove_client_side_component(game, csc_id);
            }
            game.speeches.remove(game_object_id);
        }
    }
}
impl GameObjectId {
    pub fn speak_safe<S: Into<String>>(&self, game: &mut Game, to_say: S, say_until: Option<u32>) {
        if !self.is_deleted(&game.game_objects) {
            // TODO: broken // TODO: in future, write more helpful descriptions...
            let mut to_remove = None;
            let new_id = self.add_client_side_component(
                game,
                wolf_interface::CreateComponentData::Speech(to_say.into()),
            );
            // Todo: maybe allow saying multiple things instead of overwriting
            if let Some(existing_speech) = game.speeches.get_mut(*self) {
                to_remove = Some(existing_speech.client_side_component_id);
                existing_speech.client_side_component_id = new_id;
                existing_speech.say_until = say_until;
            } else {
                game.speeches.insert(
                    *self,
                    Speech {
                        say_until,
                        client_side_component_id: new_id,
                    },
                );
            }
            if let Some(id) = to_remove {
                self.remove_client_side_component(game, id);
            }
        }
    }
}
