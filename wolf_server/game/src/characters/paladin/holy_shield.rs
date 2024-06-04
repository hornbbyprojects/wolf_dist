use super::*;
pub const SHIELD_TIME: u32 = 50;

pub struct HolyShieldAbility {
    ability_id: AbilityId,
}

#[derive(Clone)]
pub struct HolyShieldComponent {
    component_id: ComponentId,
    ported: Rc<RefCell<bool>>,
}
impl Component for HolyShieldComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_about_to_be_damaged_signal_listener(game, self.component_id);
        owner.remove_component(game, self.component_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
pub fn add_after_images(
    game: &mut Game,
    to_copy_id: GameObjectId,
    start_coords: PixelCoords,
    end_coords: PixelCoords,
) {
    if start_coords.get_plane() != end_coords.get_plane() {
        return;
    }
    let mut sprite = None;
    {
        // TODO: make this nicer...
        let to_copy = game.game_objects.get(to_copy_id).unwrap();
        for (_id, csc) in to_copy.client_side_components.iter() {
            if let CreateComponentData::Drawable(ref d) = csc.data {
                sprite = Some(d.sprite);
                break;
            }
        }
    }
    if let Some(sprite) = sprite {
        let distance = start_coords.get_distance_to(&end_coords);
        let angle = start_coords.get_direction_to(&end_coords);
        let num_images = (distance / AFTER_IMAGE_EVERY).floor() as u32;
        let mut spawn_after_image = |dist_along: f64, lifespan: u32| {
            let create_at = start_coords.offset_direction(angle, dist_along);
            let owner_id = GameObject::create_game(game, create_at);
            BasicDrawingComponent::add_to(game, owner_id, sprite, PROJECTILE_DEPTH);
            TimerSystem::add_timer(
                game,
                Box::new(move |game| {
                    owner_id.remove(game);
                }),
                lifespan,
            );
        };
        for i in 1..num_images {
            let dist_along = AFTER_IMAGE_EVERY * i as f64;
            let lifespan = AFTER_IMAGE_LIFESPAN + AFTER_IMAGE_LIFESPAN_INCREMENT * i;
            spawn_after_image(dist_along, lifespan);
        }
        spawn_after_image(
            distance,
            AFTER_IMAGE_LIFESPAN + AFTER_IMAGE_LIFESPAN_INCREMENT * num_images,
        )
    }
}
impl AboutToBeDamagedSignalListener for HolyShieldComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn AboutToBeDamagedSignalListener> {
        Box::new(self.clone())
    }
    fn receive_about_to_be_damaged_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        _damager_id: GameObjectId,
        firer_id: GameObjectId,
    ) -> OrBool {
        if let Ok(mut ported) = self.ported.try_borrow_mut() {
            if !*ported {
                *ported = true;
                let to_port = if let Some(mount) = game
                    .movement_system
                    .intend_move_system
                    .mounts_by_mounter
                    .get(&owner_id)
                {
                    mount.iter().next().unwrap().1.mounted_id
                } else {
                    owner_id
                };
                let start_coords = to_port.get_coords_game(game);
                let firer_coords = firer_id.get_coords_game(game);
                let direction = firer_coords.get_direction_to(&start_coords);
                let teleport_coords = firer_coords.offset_direction(direction, PUNISH_SPACING);
                to_port.move_to_game(game, teleport_coords);
                add_after_images(game, to_port, start_coords, teleport_coords);
                if to_port != owner_id {
                    add_after_images(game, owner_id, start_coords, teleport_coords);
                }
            }
        }
        OrBool(true) // Block
    }
}
impl HolyShieldComponent {
    fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let component = HolyShieldComponent {
            component_id,
            ported: Rc::new(RefCell::new(false)),
        };
        owner_id.add_about_to_be_damaged_signal_listener(game, component.clone());
        owner_id.add_component(game, component);
        component_id
    }
}
impl HolyShieldAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        HolyShieldAbility { ability_id }
    }
}
impl Ability for HolyShieldAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }

    fn activate(&mut self, game: &mut Game, caster: GameObjectId, _target_coords: PixelCoords) {
        let holy_shield_component_id = HolyShieldComponent::add_to(game, caster);
        let colour_component_id = add_colour(game, caster, 255, 255, 255);
        TimerSystem::add_timer(
            game,
            Box::new(move |game| {
                caster.remove_component(game, holy_shield_component_id);
                caster.remove_component(game, colour_component_id);
            }),
            SHIELD_TIME,
        );
    }
}
