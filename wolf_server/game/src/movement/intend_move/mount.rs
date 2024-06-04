use super::*;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Mount {
    pub mounter_id: GameObjectId,
    pub mounted_id: GameObjectId,
}

impl Mount {
    fn new(game: &mut Game, mounter_id: GameObjectId, mounted_id: GameObjectId) -> MountId {
        let mount = Mount {
            mounter_id,
            mounted_id,
        };
        let mount_id = game.get_id();
        // Insert by mount ID
        game.movement_system
            .intend_move_system
            .mounts
            .insert(mount_id, mount.clone());

        // Insert by mounter ID
        let already_mounter_of_entry = game
            .movement_system
            .intend_move_system
            .mounts_by_mounter
            .entry(mounter_id);
        let already_mounter_of = already_mounter_of_entry.or_insert_with(IdMap::new);
        already_mounter_of.insert(mount_id, mount.clone());

        // Insert by mounted ID
        let already_mounted_of_entry = game
            .movement_system
            .intend_move_system
            .mounts_by_mounted
            .entry(mounted_id);
        let already_mounted_of = already_mounted_of_entry.or_insert_with(IdMap::new);
        already_mounted_of.insert(mount_id, mount);

        mount_id
    }
    fn remove(game: &mut Game, id: MountId) -> Option<Mount> {
        if let Some(mount) = game.movement_system.intend_move_system.mounts.remove(id) {
            if let Entry::Occupied(mut occ) = game
                .movement_system
                .intend_move_system
                .mounts_by_mounter
                .entry(mount.mounter_id)
            {
                let also = occ.get_mut();
                also.remove(id);
                if also.is_empty() {
                    occ.remove();
                }
            }
            if let Entry::Occupied(mut occ) = game
                .movement_system
                .intend_move_system
                .mounts_by_mounted
                .entry(mount.mounted_id)
            {
                let also = occ.get_mut();
                also.remove(id);
                if also.is_empty() {
                    occ.remove();
                }
            }
            Some(mount)
        } else {
            None
        }
    }
}

pub struct MounterComponent {
    mount_id: MountId,
    component_id: ComponentId,
    mounted_component_id: ComponentId,
}

impl MounterComponent {
    pub fn add_to(game: &mut Game, mounter_id: GameObjectId, mounted_id: GameObjectId) {
        let component_id = game.get_id();
        let mount_id = Mount::new(game, mounter_id, mounted_id);
        let mounted_component_id = MountedComponent::add_to(game, mount_id, component_id);
        let component = MounterComponent {
            mount_id,
            component_id,
            mounted_component_id,
        };
        mounter_id.add_component(game, component)
    }
}

impl Component for MounterComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        if let Some(mount) = Mount::remove(game, self.mount_id) {
            mount
                .mounted_id
                .remove_component(game, self.mounted_component_id);
        }
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

pub struct MountedComponent {
    mount_id: MountId,
    component_id: ComponentId,
    mounter_component_id: ComponentId,
}

impl MountedComponent {
    /// Only to be added by MounterComponent!!
    fn add_to(
        game: &mut Game,
        mount_id: MountId,
        mounter_component_id: ComponentId,
    ) -> ComponentId {
        let component_id = game.get_id();
        let mount = game
            .movement_system
            .intend_move_system
            .mounts
            .get(mount_id)
            .unwrap();
        let owner_id = mount.mounted_id;
        let component = MountedComponent {
            mount_id,
            component_id,
            mounter_component_id,
        };
        owner_id.add_component(game, component);
        component_id
    }
}

impl Component for MountedComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        if let Some(mount) = Mount::remove(game, self.mount_id) {
            mount
                .mounter_id
                .remove_component(game, self.mounter_component_id);
        }
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
