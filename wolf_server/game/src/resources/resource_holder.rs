use super::*;
use crate::combinable::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(DropResources, &mut Game);
define_signal_listener!(TransferResources, &mut Game, transfer_to: GameObjectId);
define_signal_listener!(AddResources, &mut Game, resources: &mut Resources);
define_signal_listener!(HasResources, &Game, resources: &Resources -> OrBool);
define_signal_listener!(HasAnyResources, &Game -> OrBool);
define_signal_listener!(SpendResources, &mut Game, resources: &mut Resources);
define_signal_listener!(GetResources, &Game -> Added<Resources>);

pub struct ResourceHolder {
    pub resources: Resources,
}

impl ResourceHolder {
    pub fn new(game: &mut Game) -> ResourceHolderId {
        let id = game.get_id();
        let resource_holder = ResourceHolder {
            resources: Resources::new(),
        };
        game.resource_system
            .resource_holders
            .insert(id, resource_holder);
        id
    }
    pub fn remove(game: &mut Game, id: ResourceHolderId) {
        game.resource_system.resource_holders.remove(id);
    }
}

#[derive(Clone)]
pub struct ResourceHolderComponent {
    component_id: ComponentId,
    pub resource_holder_id: ResourceHolderId,
}

impl ResourceHolderComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ResourceHolderComponent {
        let component_id = game.get_id();
        let resource_holder_id = ResourceHolder::new(game);
        let comp = ResourceHolderComponent {
            component_id,
            resource_holder_id,
        };
        owner_id.add_drop_resources_signal_listener(game, comp.clone());
        owner_id.add_add_resources_signal_listener(game, comp.clone());
        owner_id.add_transfer_resources_signal_listener(game, comp.clone());
        owner_id.add_has_resources_signal_listener(game, comp.clone());
        owner_id.add_has_any_resources_signal_listener(game, comp.clone());
        owner_id.add_spend_resources_signal_listener(game, comp.clone());
        owner_id.add_get_resources_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp.clone());
        comp
    }
}

impl Component for ResourceHolderComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_drop_resources_signal_listener(game, self.component_id);
        owner_id.remove_add_resources_signal_listener(game, self.component_id);
        owner_id.remove_transfer_resources_signal_listener(game, self.component_id);
        owner_id.remove_has_resources_signal_listener(game, self.component_id);
        owner_id.remove_has_any_resources_signal_listener(game, self.component_id);
        owner_id.remove_spend_resources_signal_listener(game, self.component_id);
        owner_id.remove_get_resources_signal_listener(game, self.component_id);
        ResourceHolder::remove(game, self.resource_holder_id);
    }
}

impl DropResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn DropResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_drop_resources_signal(&self, game: &mut Game, owner_id: GameObjectId) {
        let coords = owner_id.get_coords_game(game);
        let resources = {
            let resource_holder = game
                .resource_system
                .resource_holders
                .get_mut(self.resource_holder_id)
                .unwrap();
            std::mem::replace(&mut resource_holder.resources, Resources::new())
        };
        create_resource_pickup(game, coords, resources);
    }
}

impl AddResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn AddResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_add_resources_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        resources: &mut Resources,
    ) {
        let resource_holder = game
            .resource_system
            .resource_holders
            .get_mut(self.resource_holder_id)
            .unwrap();
        resource_holder.resources.take_all_from(resources);
    }
}

impl TransferResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn TransferResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_transfer_resources_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        transfer_to: GameObjectId,
    ) {
        let mut resources = {
            let resource_holder = game
                .resource_system
                .resource_holders
                .get_mut(self.resource_holder_id)
                .unwrap();
            std::mem::replace(&mut resource_holder.resources, Resources::new())
        };
        transfer_to.send_add_resources_signal(game, &mut resources);
    }
}

//todo: make this check all combined, not orbool
impl HasResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn HasResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_has_resources_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
        resources: &Resources,
    ) -> OrBool {
        let resource_holder = game
            .resource_system
            .resource_holders
            .get(self.resource_holder_id)
            .unwrap();
        for (resource_type, resource_amount) in resources.resource_amounts.iter() {
            let our_amount = resource_holder
                .resources
                .resource_amounts
                .get(resource_type)
                .map(|x| *x)
                .unwrap_or(ResourceAmount(0));
            if our_amount < *resource_amount {
                return OrBool(false);
            }
        }
        OrBool(true)
    }
}
impl HasAnyResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn HasAnyResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_has_any_resources_signal(&self, game: &Game, _owner_id: GameObjectId) -> OrBool {
        let resource_holder = game
            .resource_system
            .resource_holders
            .get(self.resource_holder_id)
            .unwrap();
        OrBool(!resource_holder.resources.is_empty())
    }
}

impl SpendResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn SpendResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_spend_resources_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        resources: &mut Resources,
    ) {
        let resource_holder = game
            .resource_system
            .resource_holders
            .get_mut(self.resource_holder_id)
            .unwrap();
        resource_holder.resources.spend(resources);
    }
}

impl GetResourcesSignalListener for ResourceHolderComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetResourcesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_resources_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
    ) -> Added<Resources> {
        let resource_holder = game
            .resource_system
            .resource_holders
            .get(self.resource_holder_id)
            .unwrap();
        Added(resource_holder.resources.clone())
    }
}

impl GameObjectId {
    pub fn drop_resources(&self, game: &mut Game) {
        self.send_drop_resources_signal(game);
    }
    pub fn spend_resources(&self, game: &mut Game, mut resources: Resources) -> bool {
        let has_resources = self
            .send_has_resources_signal(game, &resources)
            .map(|x| x.extract())
            .unwrap_or(false);
        if has_resources {
            self.send_spend_resources_signal(game, &mut resources);
        }
        has_resources
    }
    pub fn has_resources(&self, game: &Game, resources: &Resources) -> bool {
        self.send_has_resources_signal(game, resources)
            .map(|x| x.extract())
            .unwrap_or(false)
    }
    pub fn get_resources(&self, game: &Game) -> Resources {
        self.send_get_resources_signal(game)
            .map(|x| x.extract())
            .unwrap_or(Resources::new())
    }
}
