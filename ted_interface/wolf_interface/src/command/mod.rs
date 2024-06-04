mod move_command;
pub use move_command::*;
mod ability_command;
pub use ability_command::*;

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub enum Command {
    Move(MoveCommand),
    Ability(AbilityCommand),
    TraverseDoorsCommand,
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn move_command() {
        let move_command = MoveCommand { dx: 3.0, dy: 4.0 };
        let command = Command::Move(move_command.clone());
        let mut buffer = Vec::new();
        command
            .clone()
            .wolf_serialise(&mut buffer)
            .expect("Failed to serialise!");
        let new_command =
            Command::wolf_deserialise(&mut buffer.as_slice()).expect("Failed to deserialise!");
        assert_eq!(command, new_command);
    }
}
