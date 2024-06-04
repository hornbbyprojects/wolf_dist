use super::*;

pub const DESERT_QUEST_START_NOTIFICATION_ID: NotificationId = NotificationId(0);
pub const DESERT_QUEST_DISTANCE_NOTIFICATION_ID: NotificationId = NotificationId(1);
pub const DESERT_QUEST_COMPLETE_NOTIFICATION_ID: NotificationId = NotificationId(2);

impl PlayerId {
    pub fn send_notification(
        &self,
        current_tick: u32,
        player_system: &mut PlayerSystem,
        id: NotificationId,
        message: String,
    ) {
        if let Some(player) = player_system.players.get_mut(*self) {
            match player.notifications.entry(id) {
                hash_map::Entry::Occupied(mut occ) => {
                    occ.get_mut().message = message;
                }
                hash_map::Entry::Vacant(mut vac) => {
                    vac.insert(Notification {
                        message,
                        sent_at: current_tick,
                    });
                }
            }
            player.send_notification_server_message();
        }
    }
    pub fn clear_notification(&self, player_system: &mut PlayerSystem, id: NotificationId) {
        if let Some(player) = player_system.players.get_mut(*self) {
            player.notifications.remove(id);
            player.send_notification_server_message();
        }
    }
}
impl Player {
    pub fn send_notification_server_message(&mut self) {
        self.server_messages
            .push(ServerMessage::SetNotifications(SetNotificationsMessage {
                notifications: self.notifications.values().map(Clone::clone).collect(),
            }));
    }
}

impl GameObjectId {
    pub fn send_notification(
        &self,
        current_tick: u32,
        player_system: &mut PlayerSystem,
        notification_id: NotificationId,
        message: String,
    ) {
        if let Some(player_id) = player_system.players_by_game_object.get(*self) {
            let moved_player_id = *player_id;
            moved_player_id.send_notification(current_tick, player_system, notification_id, message)
        }
    }
    pub fn clear_notification(
        &self,
        player_system: &mut PlayerSystem,
        notification_id: NotificationId,
    ) {
        if let Some(player_id) = player_system.players_by_game_object.get(*self) {
            let moved_player_id = *player_id;
            moved_player_id.clear_notification(player_system, notification_id);
        }
    }
}
