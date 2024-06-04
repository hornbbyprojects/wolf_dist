use super::{
    slash_animation::{SlashAnimation, SlashAnimationComponent},
    *,
};
use coords::{Angle, PixelCoords};
use id::*;
use wolf_interface::*;

pub struct Game {
    id_counter: u32,
    pub tick_counter: u32,
    pub current_scaling: f64,
    pub current_view_coords: PixelCoords,
    pub current_bound_object: Option<GameObjectId>,
    pub game_objects: IdMap<GameObjectId, GameObject>,
    pub component_info: IdMap<ComponentId, ComponentInfo>,
    pub drawables: IdMap<DrawableId, Drawable>,
    pub slash_animations: IdMap<SlashAnimationId, SlashAnimation>,
    pub to_delete: Vec<GameObjectId>,
    pub currently_saying: IdMap<GameObjectId, (ComponentId, String)>,
    pub notifications: Vec<Notification>,
}

impl Game {
    pub fn get_id<T: From<u32>>(&mut self) -> T {
        let id = self.id_counter;
        self.id_counter += 1;
        id.into()
    }
    pub fn new() -> Self {
        Game {
            id_counter: 0,
            current_scaling: 1.0,
            tick_counter: 0,
            current_bound_object: None,
            current_view_coords: PixelCoords::new_at_zero(),
            game_objects: IdMap::new(),
            component_info: IdMap::new(),
            drawables: IdMap::new(),
            slash_animations: IdMap::new(),
            to_delete: Vec::new(),
            currently_saying: IdMap::new(),
            notifications: Vec::new(),
        }
    }
    pub fn step(&mut self) {
        //handle client side procedural updating here
    }
    pub fn update_game_objects(&mut self, msg: UpdateGameObjectsMessage) {
        self.tick_counter = msg.current_tick;
        self.current_view_coords = msg.view_message.view_coords;
        self.current_bound_object = msg.view_message.watching_object_id.map(|x| x.into());
        for remove_game_object in msg.deleted_game_objects.iter() {
            let game_object_id: GameObjectId = remove_game_object.game_object_id.into();
            game_object_id.remove(self);
        }
        for move_game_object in msg.updated_game_objects {
            let existing = self
                .game_objects
                .entry(move_game_object.game_object_id.into());
            let game_object = existing.or_insert_with(GameObject::new);
            game_object.coords = move_game_object.coords;
            if move_game_object.rotation.0 > 0 {
                let angle: Angle = move_game_object.rotation.clone().into();
            }
            game_object.rotation = move_game_object.rotation.into();
        }
    }
    pub fn update_components(&mut self, msg: UpdateComponentsMessage) {
        for update_components_for_object_message in msg.updates_by_object {
            self.handle_update_components_for_object_message(update_components_for_object_message);
        }
    }
    pub fn handle_update_components_for_object_message(
        &mut self,
        msg: UpdateComponentsForObjectMessage,
    ) {
        let game_object_id: GameObjectId = msg.game_object_id.into();
        for create_component_message in msg.created_components {
            self.handle_create_component_message(game_object_id, create_component_message);
        }
        for update_component_message in msg.updated_components {
            self.handle_update_component_message(game_object_id, update_component_message);
        }
        for remove_component_message in msg.removed_components {
            self.handle_remove_component_message(game_object_id, remove_component_message);
        }
    }
    pub fn handle_create_component_message(
        &mut self,
        game_object_id: GameObjectId,
        msg: CreateComponentMessage,
    ) {
        let component_id: ComponentId = msg.component_id.into();
        match msg.data {
            CreateComponentData::Drawable(drawable_data) => {
                DrawableComponent::add_to(
                    self,
                    game_object_id,
                    component_id,
                    drawable_data.sprite,
                    drawable_data.depth,
                );
            }

            CreateComponentData::HealthBar => {
                HealthBarComponent::add_to(self, game_object_id, component_id);
            }
            CreateComponentData::HealthProportionTenThousandths(ten_thousandths) => {
                let proportion = (ten_thousandths as f64) / 10_000.0;
                HealthProportionComponent::add_to(self, game_object_id, component_id, proportion);
            }
            CreateComponentData::Coloured(coloured_data) => {
                ColouredComponent::add_to(
                    self,
                    game_object_id,
                    component_id,
                    coloured_data.r,
                    coloured_data.g,
                    coloured_data.b,
                );
            }
            CreateComponentData::WideVision => {
                WideVisionComponent::add_to(self, game_object_id, component_id);
            }
            CreateComponentData::SlashAnimation(sad) => {
                SlashAnimationComponent::add_to(self, game_object_id, component_id, sad.start_tick);
            }
            CreateComponentData::Speech(to_say) => {
                self.currently_saying
                    .insert(game_object_id, (component_id, to_say));
                self.component_info
                    .insert(component_id, ComponentInfo::Speech);
            }
        }
    }
    pub fn handle_update_component_message(
        &mut self,
        _game_object_id: GameObjectId,
        _msg: UpdateComponentMessage,
    ) {
        unimplemented!()
    }
    pub fn handle_remove_component_message(
        &mut self,
        game_object_id: GameObjectId,
        msg: RemoveComponentMessage,
    ) {
        if let Some(component_info) = self.component_info.remove(msg.component_id.into()) {
            match component_info {
                ComponentInfo::Speech => {
                    let entry = self.currently_saying.entry(game_object_id);
                    match entry {
                        std::collections::hash_map::Entry::Occupied(occ) => {
                            if occ.get().0 == msg.component_id.into() {
                                occ.remove();
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(_) => {}
                    }
                }
            };
        }
        if let Some(game_object) = self.game_objects.get_mut(game_object_id) {
            if let Some(component) = game_object.components.remove(msg.component_id.into()) {
                component.on_remove(self, game_object_id);
            }
        }
    }
}
