
use super::*;

const MINING_COST: u32 = 100;
const HOUSE_COST: u32 = 100;

enum Action {
    Mine(Box<Resources>),
    BuildHouse,
}

fn build_house_action(node: &Node) -> Option<Node> {
    if node.goals_state.house_needed {
        let mut new_goals_state = node.goals_state.clone();
        new_goals_state.house_needed = false;
        new_goals_state.need_resources(Resources::wood(100));
        let mut new_node = node.clone();
        new_node.goals_state = new_goals_state;
        new_node.cost += HOUSE_COST;
        return Some(new_node);
    }
    None
}

fn mine_action(node: &Node) -> Option<Node> {
    if node.goals_state.resources_needed.is_some() {
        let mut new_goals_state = node.goals_state.clone();
        new_goals_state.resources_needed = None;
        let mut new_node = node.clone();
        new_node.goals_state = new_goals_state;
        new_node.cost += MINING_COST;
        new_node.action_stack.push(Action::Mine(node.goals_state.resources_needed.clone()));
        return Some(new_node);
    }
    None
}

fn traverse_edges(node: &Node) -> Vec<Node> {
    mine_action(node).chain(build_house_action(node)).collect()
}

struct MinePlan {
    resources_needed: Resources,
    current_target: Option<HarvestableID>
}
impl MinePlan {
    fn step(game: &mut Game, owner: GameObjectId) -> PlanResult {
        if let Some(current_target_id) = self.current_target {
            if let target = game.resource_system.harvestables.get(current_target_id);
        owner.send_move_to_signal(
        PlanResult::Continue
        }
    }
}

