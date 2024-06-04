use coords::PixelCoords;

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct ViewMessage {
    pub watching_object_id: Option<u32>,
    pub view_coords: PixelCoords,
}
