use super::*;

/*
examples:
zombie is linked to undead
undead is friends with player (undead charm)
player is linked to minions
player is friends with player 2 (flag)
zombie can attack player 2
player 2 can attack zombies
player 1 & minions can attack no-one
*/

//both friends and linkers should enforce symettry
pub struct Allegiance {
    pub friends: WolfHashSet<AllegianceId>, //we won't hurt these people
    pub linkers: WolfHashSet<AllegianceId>, //these people's friends are our friends, we effectively "are" them. A zombie represents the whole undead horde!
    pub friendly_fire: bool,
}
impl Allegiance {
    pub fn new_on_list(
        allegiance_list: &mut AllegianceList,
        linkers: WolfHashSet<AllegianceId>,
    ) -> AllegianceId {
        let mut friends = WolfHashSet::new();
        for other_allegiance_id in linkers.iter() {
            friends.insert(*other_allegiance_id);
        }
        let allegiance = Allegiance {
            friends,
            linkers: linkers.clone(),
            friendly_fire: false,
        };
        let allegiance_id = allegiance_list.add(allegiance);
        for other_allegiance_id in linkers.iter() {
            let other_allegiance = allegiance_list.get_mut(*other_allegiance_id).unwrap();
            other_allegiance.friends.insert(allegiance_id);
            other_allegiance.linkers.insert(allegiance_id);
        }
        allegiance_id
    }
    pub fn new(game: &mut Game, linkers: WolfHashSet<AllegianceId>) -> AllegianceId {
        Self::new_on_list(&mut game.allegiance_system.allegiance_list, linkers)
    }
    pub fn remove(game: &mut Game, id: AllegianceId) {
        if let Some(allegiance) = game.allegiance_system.allegiance_list.remove(id) {
            for friend_id in allegiance.friends {
                let friend = game
                    .allegiance_system
                    .allegiance_list
                    .get_mut(friend_id)
                    .unwrap();
                friend.friends.remove(&id);
                friend.linkers.remove(&id);
            }
        }
    }
    pub fn link(allegiance_list: &mut AllegianceList, first: AllegianceId, second: AllegianceId) {
        let first_allegiance = allegiance_list.get_mut(first).unwrap();
        first_allegiance.friends.insert(second);
        first_allegiance.linkers.insert(second);
        let second_allegiance = allegiance_list.get_mut(second).unwrap();
        second_allegiance.friends.insert(first);
        second_allegiance.linkers.insert(first);
    }
    //wipes all current allegiances etc
    pub fn reset(allegiance_list: &mut AllegianceList, id: AllegianceId) {
        let allegiance = allegiance_list.get_mut(id).unwrap();
        allegiance.linkers = WolfHashSet::new();
        let friends = std::mem::replace(&mut allegiance.friends, WolfHashSet::new());
        for allegiance_id in friends {
            let other_allegiance = allegiance_list.get_mut(allegiance_id).unwrap();
            other_allegiance.friends.remove(&allegiance_id);
            other_allegiance.linkers.remove(&allegiance_id);
        }
    }
}
