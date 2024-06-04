use rand::thread_rng;
use wolf_hash_map::WolfHashMap;

use crate::hunting::HuntingSystem;

use super::*;

const SQUADPORT_DISTANCE: f64 = 200.0;
const HUNT_RADIUS: f64 = 600.0;
const SQUAD_REPULSION_FORCE: f64 = 2500.0;

pub struct Squad {
    pub forces: WolfHashMap<GameObjectId, (f64, f64)>,
    pub target_position: PixelCoords,
    pub members: Vec<GameObjectId>,
    pub flag: GameObjectId,
}

impl Squad {
    pub fn step(game: &mut Game) {
        let mut flag_movements = Vec::new();
        for (_id, squad) in game.villages_system.squads.iter_mut() {
            // If squad not reached point, next squad
            let mut new_position = true;
            let mut forces = WolfHashMap::new();
            for (i, member) in squad.members.iter().enumerate() {
                if let Some(pos) = member.get_coords_safe(&game.game_objects) {
                    for j in i..squad.members.len() {
                        let other_member = squad.members[j];
                        if let Some(other_pos) = other_member.get_coords_safe(&game.game_objects) {
                            if other_pos.get_plane() == pos.get_plane() {
                                let distance = pos.get_distance_to(&other_pos);
                                let (magnitude, direction) = if distance == 0.0 {
                                    let direction =
                                        Angle::enforce_range(thread_rng().gen_range(-PI..PI));
                                    let magnitude = SQUAD_REPULSION_FORCE;
                                    (magnitude, direction)
                                } else {
                                    let direction = pos.get_direction_to(&other_pos);
                                    let magnitude = (SQUAD_REPULSION_FORCE / (distance * distance))
                                        .min(SQUAD_REPULSION_FORCE);
                                    (magnitude, direction)
                                };

                                let fx = direction.cos() * magnitude;
                                let fy = direction.sin() * magnitude;
                                let existing = forces.entry(*member).or_insert((0.0, 0.0));
                                existing.0 -= fx;
                                existing.1 -= fy;

                                let other_existing =
                                    forces.entry(other_member).or_insert((0.0, 0.0));
                                other_existing.0 += fx;
                                other_existing.1 += fy;
                            }
                        }
                    }
                    if pos.get_plane() != squad.target_position.get_plane() {
                        // Hopelessly lost
                        continue;
                    }
                    if pos.get_distance_to(&squad.target_position) > SQUADPORT_DISTANCE {
                        new_position = false;
                    }
                }
            }
            squad.forces = forces;
            if new_position {
                let x = thread_rng().gen_range(-500..500);
                let y = thread_rng().gen_range(-500..500);
                squad.target_position =
                    PixelCoords::new_to_fixed(squad.target_position.get_plane(), x, y);
            }
            flag_movements.push((squad.flag, squad.target_position));
        }
        for (flag, pos) in flag_movements {
            flag.move_to_game(game, pos);
        }
    }
}
pub struct GuardMind {
    squad_id: SquadId,
    owner_id: GameObjectId,
    hunter_behaviour: BehaviourId,
}

impl GuardMind {
    pub fn step(game: &mut Game) {
        let mut squadporters = Vec::new();
        let mut hunters = Vec::new();
        for (id, guard_mind) in game.villages_system.guard_minds.iter() {
            let mind = game.behaviour_system.minds.get(id).unwrap();
            if mind.active_behaviour.is_none() {
                let squad = game
                    .villages_system
                    .squads
                    .get(guard_mind.squad_id)
                    .unwrap();
                let my_coords = guard_mind.owner_id.get_coords(&game.game_objects);
                if squad.target_position.get_plane() != my_coords.get_plane() {
                    // Boy, Jimmy, I think we're lost....
                    continue;
                }
                let distance = my_coords.get_distance_to(&squad.target_position);
                if distance > SQUADPORT_DISTANCE {
                    let direction = my_coords.get_direction_to(&squad.target_position);
                    let mut offset = PixelCoords::new_at_zero().offset_direction(direction, 5.0);
                    if let Some((fx, fy)) = squad.forces.get(&mind.game_object_id) {
                        offset = offset.translate(*fx, *fy);
                    }
                    let offset_direction = PixelCoords::new_at_zero().get_direction_to(&offset);
                    squadporters.push((guard_mind.owner_id, offset_direction));
                } else {
                    if let Some(prey) =
                        HuntingSystem::get_closest_prey(game, guard_mind.owner_id, HUNT_RADIUS)
                    {
                        hunters.push((id, prey));
                    }
                }
            }
        }
        for (id, direction) in squadporters {
            id.intend_move_in_direction_minimal(
                &mut game.movement_system.intend_move_system,
                direction,
            )
        }
        for (mind_id, prey_id) in hunters {
            let behaviour_id = game
                .villages_system
                .guard_minds
                .get(mind_id)
                .unwrap()
                .hunter_behaviour;
            HunterBehaviour::set_target(game, behaviour_id, prey_id);
            mind_id.set_active_behaviour(&mut game.behaviour_system.minds, Some(behaviour_id));
            let mind = game.behaviour_system.minds.get_mut(mind_id).unwrap();
            mind.active_behaviour = Some(behaviour_id);
        }
    }
}

pub struct GuardMindComponent {
    component_id: ComponentId,
    hunter_behaviour_component: ComponentId,
    mind_id: MindId,
}

impl Component for GuardMindComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        game.villages_system.guard_minds.remove(self.mind_id);
        owner.remove_component(game, self.hunter_behaviour_component);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
impl GuardMindComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, squad_id: SquadId) {
        let component_id = game.get_id();
        let mind_id = MindComponent::add_to(game, owner_id);
        let hunter_behaviour_component = HunterBehaviourComponent::add_to(game, owner_id, mind_id);
        let guard_mind = GuardMind {
            squad_id,
            owner_id,
            hunter_behaviour: hunter_behaviour_component.hunter_behaviour,
        };
        let comp = GuardMindComponent {
            component_id,
            hunter_behaviour_component: hunter_behaviour_component.component_id,
            mind_id,
        };
        owner_id.add_component(game, comp);
        game.villages_system.guard_minds.insert(mind_id, guard_mind);
    }
}

pub fn spawn_guard(game: &mut Game, squad_id: SquadId, coords: PixelCoords) -> GameObjectId {
    let owner_id = GameObject::create_game(game, coords);
    AllegianceComponent::add_to(
        game,
        owner_id,
        vec![
            game.allegiance_system
                .special_allegiances
                .villager_allegiance,
        ],
    );
    GuardMindComponent::add_to(game, owner_id, squad_id);
    BasicAbilityUserComponent::add_to(game, owner_id, vec![AbilityTypeId::FireballId]);
    add_health_bar(game, owner_id);
    WalkerComponent::add_to(game, owner_id, 1.0, 0.5);
    DamageableComponent::add_to(game, owner_id);
    BasicDrawingComponent::add_to(game, owner_id, VILLAGER_SPRITE, DEFAULT_DEPTH);
    DieOnNoHealthComponent::add_to(game, owner_id);
    DeleteOnDeathComponent::add_to(game, owner_id);
    owner_id
}
pub fn spawn_guard_squad(game: &mut Game) {
    let squad_id = game.get_id();
    let mut members = Vec::new();
    for _ in 0..5 {
        let dx = thread_rng().gen_range(-256..256);
        let dy = thread_rng().gen_range(-256..256);
        members.push(spawn_guard(
            game,
            squad_id,
            PixelCoords::new_to_fixed(Plane(0), dx, dy),
        ));
    }
    let flag = GameObject::create_game(game, PixelCoords::new_at_zero());
    BasicDrawingComponent::add_to(game, flag, FLAG_SPRITE, DEFAULT_DEPTH);
    let squad = Squad {
        members,
        target_position: PixelCoords::new_at_zero(),
        forces: WolfHashMap::new(),
        flag,
    };
    game.villages_system.squads.insert(squad_id, squad);
}
