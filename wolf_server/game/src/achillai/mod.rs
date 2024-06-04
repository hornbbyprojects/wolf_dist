mod world_state;
pub use world_state::*;
mod goals;
pub use goals::*;
mod actions;
pub use actions::*;

pub struct Node {
    action_stack: Vec<Action>,
    state: AiState,
    cost: u32,
}
impl Node {
    fn get_total_estimated_cost(&self) -> u32 {
        self.cost + self.state.goals_state.estimated_cost()
    }
}

pub struct NodesToVisit {
    inner: Vec<Node>,
}
impl NodesToVisit {
    fn pop(&mut self) -> Node {
        //O(1)
        self.inner.pop()
    }
    fn insert(&mut self, node: Node) {
        //O(log n)
        let to_insert_at = self
            .inner
            .binary_search_by_key(node.get_total_estimated_cost(), |other_node| {
                other_node.get_total_estimated_cost()
            })
            .map_or_else(|x| x, |x| x);
        //O(n)
        self.inner.insert(to_insert_at, node);
    }
    fn reprioritise(&mut self, node: Node, _old_cost: u32) {
        // O(n)
        if let Some(old_position) = self
            .inner
            .iter()
            .position(|other_node| other_node.state == node.state)
        {
            self.inner.remove(old_position);
        }
        //O(n)
        self.insert(node);
    }
}


///Allows for pausing the planning process to spread across multiple ticks
pub struct AiCalc {
    current_node: Node,
    nodes_to_visit: NodesToVisit,
    previous_costs: WolfHashMap<AiState, u32>
}
pub enum CalcResult {
    Failure,
    Success(Node),
    Continue(AiCalc),
}
pub fn start_plan(starting_state: AiState) -> AiCalc {
    let mut current_node = Node {
        state: starting_state,
        cost: 0,
        parent: None,
    };
    let mut nodes_to_visit = Vec::new();
    let mut previous_costs = WolfHashMap::new();
    previous_costs.insert(current_node.state, 0);
    AiCalc {
        current_node,
        nodes_to_visit,
        previous_costs
    }
}
pub fn step_plan(calc: AiCalc) -> CalcResult {
    let next_node = calc.nodes_to_visit.pop();
    if let Some(next_node) = next_node {
        if next_node.state.goals_state.satisfied() {
            return CalcResult::Success(next_node);
        }
        for node in traverse_edges(next_node) {
            let previous_cost = calc.previous_costs(&node.state);
            if let Some(previous_cost) = previous_cost {
                if node.cost >= previous_cost {
                    continue;
                }
                calc.previous_costs.insert(node.state.clone(), node.cost);
                calc.nodes_to_visit.reprioritise(node, previous_cost);
            } else {
                calc.previous_costs.insert(node.state.clone(), node.cost);
                calc.nodes_to_visit.insert(node);
            }
        }
        return CalcResult::Continue(calc);
    }
    else {
        return CalcResult::Failure
    }
}
