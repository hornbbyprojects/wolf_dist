use super::*;
use std::collections::*;
use std::rc::*;

#[derive(Debug)]
struct ActionNode {
    parent: Option<Rc<ActionNode>>,
    action: Option<Box<dyn ActionSeed>>,
    needs: Needs,
    state: DerivedPlannerState,
    cost: ActionCost,
}

/*
    generic over what counts as a step in the plan, usually an enum
*/
const MAX_LOOPS: usize = 200;

#[allow(dead_code)]
pub fn plan(
    game: &Game, //used for loading state
    owner_id: GameObjectId,
    starting_needs: Needs,
    action_generators: Vec<Box<dyn ActionGenerator>>, //pairs a plan step with its prereqs & effects
) -> Option<VecDeque<Box<dyn ActionSeed>>> {
    let mut active_nodes = VecDeque::new();

    let mut starting_state = StartingPlannerState::new();

    let start_action_node = Rc::new(ActionNode {
        action: None,
        parent: None,

        needs: starting_needs,
        state: DerivedPlannerState::new(),

        cost: ActionCost(0),
    });
    active_nodes.push_front(start_action_node);
    let mut current_best: Option<Rc<ActionNode>> = None;
    let mut current_cost = None;

    let mut loop_count = 0;
    while let Some(active_node) = active_nodes.pop_front() {
        //we have better options
        if current_cost.is_some() && active_node.cost >= current_cost.unwrap() {
            continue;
        }

        for action_generator in action_generators.iter() {
            let new_states = action_generator.generate_actions(
                game,
                owner_id,
                &mut starting_state,
                &active_node.state,
                &active_node.needs,
            );
            for (state, needs, cost, action) in new_states {
                let satisfied = needs.is_satisfied();
                let node = Rc::new(ActionNode {
                    parent: Some(Rc::clone(&active_node)),
                    state,
                    needs,
                    action: Some(action),

                    cost: active_node.cost + cost,
                });
                let cost_lower = current_cost.is_none() || current_cost.unwrap() > node.cost;

                if cost_lower {
                    if satisfied {
                        current_cost = Some(node.cost);
                        current_best = Some(node);
                    } else {
                        //pushed to the back so we explore all options, rather than getting stuck in a loop if e.g. action 0 is repeatable
                        //if cost is already too high, no point
                        active_nodes.push_back(node);
                    }
                }
            }
        }

        loop_count += 1;
        if loop_count > MAX_LOOPS {
            break;
        }
    }
    current_best.map(|node| {
        let mut iter_over = Some(node);
        let mut ret = VecDeque::new();
        while let Some(next) = iter_over {
            let next_owned = Rc::try_unwrap(next).unwrap();
            if let Some(action) = next_owned.action {
                ret.push_back(action);
            }
            iter_over = next_owned.parent;
        }
        ret
    })
}
