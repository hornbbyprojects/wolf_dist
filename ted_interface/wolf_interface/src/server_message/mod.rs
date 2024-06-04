mod terrain;
pub use terrain::*;
mod views;
pub use views::*;
mod client_side_component;
pub use client_side_component::*;
mod game_object;
pub use game_object::*;

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct SlotMappingMessage {
    pub slot_to_ability_icon: Vec<Option<u32>>,
}

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct Notification {
    pub message: String,
    pub sent_at: u32,
}
#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct SetNotificationsMessage {
    pub notifications: Vec<Notification>,
}
#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub enum ServerMessage {
    ChunkInfo(ChunkInfoMessage),
    UpdateGameObjects(UpdateGameObjectsMessage),
    UpdateComponents(UpdateComponentsMessage),
    ChunkUpdate(ChunkUpdateMessage),
    ChunkUnload(ChunkUnloadMessage),
    SlotMapping(SlotMappingMessage),
    SetNotifications(SetNotificationsMessage),
}

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct UpdateGameObjectsMessage {
    pub view_message: ViewMessage,
    pub current_tick: u32,
    pub updated_game_objects: Vec<MoveGameObject>,
    pub deleted_game_objects: Vec<RemoveGameObject>,
}

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct UpdateComponentsForObjectMessage {
    pub game_object_id: u32,
    pub created_components: Vec<CreateComponentMessage>,
    pub updated_components: Vec<UpdateComponentMessage>,
    pub removed_components: Vec<RemoveComponentMessage>,
}

#[derive(Debug, WolfSerialise, PartialEq, Clone)]
pub struct UpdateComponentsMessage {
    pub updates_by_object: Vec<UpdateComponentsForObjectMessage>,
}
