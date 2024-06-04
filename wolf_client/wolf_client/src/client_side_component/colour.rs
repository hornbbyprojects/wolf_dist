use super::*;

#[derive(Clone)]
pub struct ColouredComponent {
    pub component_id: ComponentId,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl MutateTextureSignalListener for ColouredComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<(dyn drawable::MutateTextureSignalListener + 'static)> {
        Box::new(self.clone())
    }
    fn receive_mutate_texture_signal(
        &self,
        _game: &game::Game,
        _owner_id: GameObjectId,
        texture: &mut sdl2::render::Texture<'_>,
    ) {
        texture.set_color_mod(self.r, self.g, self.b);
    }
}

impl Component for ColouredComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_mutate_texture_signal_listener(game, self.component_id);
    }
}

impl ColouredComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        component_id: ComponentId,
        r: u8,
        g: u8,
        b: u8,
    ) {
        let comp = ColouredComponent {
            component_id,
            r,
            g,
            b,
        };
        owner_id.add_component(game, comp.clone());
        owner_id.add_mutate_texture_signal_listener(game, comp);
    }
}
