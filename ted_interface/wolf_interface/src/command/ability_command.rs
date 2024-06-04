use coords::*;

#[derive(Debug, Clone, PartialEq, WolfSerialise)]
pub struct AbilityCommand {
    pub slot: u8,
    pub target_coords: PixelCoords,
}
