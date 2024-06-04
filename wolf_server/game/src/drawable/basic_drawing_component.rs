use std::rc::Rc;

use crate::basic_client_side_component::BasicClientSideComponent;
use crate::game::*;
use signal_listener_macro::define_signal_listener;
use wolf_hash_map::WolfHashMap;
use wolf_interface::*;

define_signal_listener!(SetFacing, &mut Game, facing: CardinalDirection);
#[derive(Clone)]
pub struct BasicDrawingComponent {}

impl BasicDrawingComponent {
    pub fn get_data(sprite: u32, depth: i32) -> CreateComponentData {
        let drawable_data = CreateDrawableData { sprite, depth };
        let data = CreateComponentData::Drawable(drawable_data);
        data
    }
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        sprite: u32,
        depth: i32,
    ) -> BasicClientSideComponent {
        let data = Self::get_data(sprite, depth);
        BasicClientSideComponent::add_to(game, owner_id, data)
    }
    pub fn update(
        game: &mut Game,
        owner: GameObjectId,
        client_side_component_id: ClientSideComponentId,
        sprite: u32,
        depth: i32,
    ) {
        let data = Self::get_data(sprite, depth);
        owner.refresh_client_side_component(game, client_side_component_id, data);
    }
}

pub struct FacingSpriteComponent {
    pub component_id: ComponentId,
    pub basic_drawing_component_id: ComponentId,
    pub client_side_component_id: ClientSideComponentId,
    pub direction_to_sprite: WolfHashMap<CardinalDirection, u32>,
    pub depth: i32,
}

impl SetFacingSignalListener for Rc<FacingSpriteComponent> {
    fn receive_set_facing_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        facing: CardinalDirection,
    ) {
        BasicDrawingComponent::update(
            game,
            owner_id,
            self.client_side_component_id,
            self.direction_to_sprite
                .get(&facing)
                .map(|x| *x)
                .unwrap_or(ERROR_SPRITE),
            self.depth,
        );
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn SetFacingSignalListener> {
        Box::new(Rc::clone(self))
    }
}

impl Component for Rc<FacingSpriteComponent> {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_component(game, self.basic_drawing_component_id);
        owner.remove_set_facing_signal_listener(game, self.component_id);
        owner.remove_sticky_facer(game);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl FacingSpriteComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        direction_to_sprite: WolfHashMap<CardinalDirection, u32>,
        depth: i32,
    ) -> ComponentId {
        owner_id.add_sticky_facer(game);
        let component_id = game.get_id();
        let basic_drawing_component = BasicDrawingComponent::add_to(
            game,
            owner_id,
            direction_to_sprite
                .get(&CardinalDirection::Right)
                .map(|x| *x)
                .unwrap_or(ERROR_SPRITE),
            depth,
        );
        let comp = Rc::new(FacingSpriteComponent {
            component_id,
            basic_drawing_component_id: basic_drawing_component.component_id,
            client_side_component_id: basic_drawing_component.client_side_component_id,
            direction_to_sprite,
            depth,
        });
        owner_id.add_component(game, Rc::clone(&comp));
        owner_id.add_set_facing_signal_listener(game, comp);
        component_id
    }
}
