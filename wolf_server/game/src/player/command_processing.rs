use super::*;
use crate::{abilities::CastAbilitySignalSender, villages::traverse_doors};
use utilities::ret_opt;

define_signal_listener!(MoveCommand, &mut Game, move_command: &MoveCommand);

impl Player {
    pub fn handle_commands(game: &mut Game) {
        let mut to_handle = Vec::new();
        for (id, player) in game.player_system.players.iter_mut() {
            if !player.commands.is_empty() {
                to_handle.push((id, std::mem::replace(&mut player.commands, Vec::new())));
            }
        }
        for (id, commands) in to_handle {
            for command in commands {
                id.process_command(game, command);
            }
        }
    }
}
impl PlayerId {
    fn get_game_object_id(&self, game: &Game) -> Option<GameObjectId> {
        let player = game.player_system.players.get(*self).unwrap();
        player.bound_object_id
    }
    fn process_command(&self, game: &mut Game, command: Command) {
        match command {
            Command::Move(mc) => self.process_move_command(game, mc),
            Command::Ability(ca) => self.process_ability_command(game, ca),
            Command::TraverseDoorsCommand => self.process_traverse_doors_command(game),
        }
    }
    fn process_traverse_doors_command(&self, game: &mut Game) {
        if let Some(game_object_id) = self.get_game_object_id(game) {
            traverse_doors(game, game_object_id);
        }
    }
    fn process_move_command(&self, game: &mut Game, move_command: MoveCommand) {
        let watching_object_id = ret_opt!(self.get_game_object_id(game));
        if move_command.dx != 0.0 || move_command.dy != 0.0 {
            let angle = PixelCoords::new_at_zero().get_direction_to(
                &PixelCoords::new_at_zero().translate(move_command.dx, move_command.dy),
            );
            watching_object_id.intend_move_in_direction(game, angle);
        } else {
            watching_object_id.intend_stop(&mut game.movement_system.intend_move_system);
        }
    }
    fn process_ability_command(&self, game: &mut Game, ability_command: AbilityCommand) {
        let watching_object_id = ret_opt!(self.get_game_object_id(game));
        if let Some(mapping) = game
            .player_system
            .slot_mappings
            .get(&(*self, watching_object_id))
        {
            if let Some(Some(ability_id)) = mapping
                .slot_to_ability_id
                .get(ability_command.slot as usize)
            {
                let target_coords = ability_command.target_coords;
                watching_object_id.send_cast_ability_signal(game, *ability_id, target_coords);
            }
        }
    }
}
