use super::*;
use crate::combinable::CombinedVecs;
use signal_listener_macro::define_signal_listener;

const AI_CONFUSION_LENGTH: u32 = 200;

define_signal_listener!(GetGoals, &Game -> CombinedVecs<Box<dyn Goal>>);
define_signal_listener!(GetActionGenerators, &Game -> CombinedVecs<Box<dyn ActionGenerator>>);

pub struct PlanAiMode {
    pub current_plan: VecDeque<Box<dyn ActionSeed>>,
    pub current_action: Option<Box<dyn Action>>,
}

pub enum AiMode {
    Plan(PlanAiMode),
    SimplePlan(Box<dyn SimplePlan>),
}

pub struct Ai {
    pub game_object_id: GameObjectId,

    //confusion is when the ai couldn't make a plan, or had no valid goals
    pub confusion_ends: Option<u32>,
    pub ai_mode: Option<AiMode>,
}

impl Ai {
    pub fn step(game: &mut Game) {
        let ai_ids: Vec<AiId> = game.ai_system.ais.iter().map(|(k, _)| k).collect();
        for ai_id in ai_ids {
            let mut failed = false;
            let mut ai = game.ai_system.ais.remove(ai_id).unwrap();
            let game_object_id = ai.game_object_id;
            if let Some(confusion_ends) = ai.confusion_ends {
                if confusion_ends > game.tick_counter {
                    game.ai_system.ais.insert(ai_id, ai);
                    continue;
                }
            }

            ai.confusion_ends = None;

            if let Some(ref mut ai_mode) = ai.ai_mode {
                match ai_mode {
                    AiMode::Plan(plan_mode) => {
                        if let Some(ref mut current_action) = plan_mode.current_action {
                            //currently stepping an action, see if we're done
                            let result = current_action.step(game, game_object_id);

                            match result {
                                ActionResult::Success => {
                                    //move on to next step in the plan
                                    plan_mode.current_action = None;
                                }
                                ActionResult::Failure => {
                                    failed = true;
                                }
                                ActionResult::Continue => {}
                            }
                        } else {
                            //get next action
                            if let Some(next_seed) = plan_mode.current_plan.pop_front() {
                                if let Some(action) = next_seed.get_action(game, game_object_id) {
                                    plan_mode.current_action = Some(action);
                                }
                            } else {
                                //Congratulations
                                ai.ai_mode = None;
                            }
                        }
                    }
                    AiMode::SimplePlan(simple_plan) => {
                        let result = simple_plan.step(game, game_object_id);
                        match result {
                            ActionResult::Continue => {}
                            ActionResult::Failure => {
                                failed = true;
                            }
                            ActionResult::Success => ai.ai_mode = None,
                        }
                    }
                }
            } else {
                //Generate new plan
                let mut max_importance = None;
                let mut current_goal_index = None;

                //first we have to get a goal
                let mut goals = game_object_id
                    .send_get_goals_signal(game)
                    .map(|x| x.extract())
                    .unwrap_or(Vec::new());

                for (index, goal) in goals.iter().enumerate() {
                    if let Some(current_importance) = goal.get_importance(game, game_object_id) {
                        if max_importance.is_none() || max_importance.unwrap() < current_importance
                        {
                            max_importance = Some(current_importance);
                            current_goal_index = Some(index);
                        }
                    }
                }

                match current_goal_index {
                    None => failed = true,
                    Some(goal_index) => {
                        let goal = goals.remove(goal_index);

                        let needs_or_simple_plan = goal.get_method(game, game_object_id);
                        match needs_or_simple_plan {
                            GoalResult::SimplePlan(simple_plan) => {
                                ai.ai_mode = Some(AiMode::SimplePlan(simple_plan));
                            }
                            GoalResult::Needs(needs) => {
                                let action_generators = game_object_id
                                    .send_get_action_generators_signal(game)
                                    .map(|x| x.extract())
                                    .unwrap_or(Vec::new());

                                let finished_plan: Option<VecDeque<Box<dyn ActionSeed>>> =
                                    plan(game, game_object_id, needs, action_generators);

                                if let Some(finished_plan) = finished_plan {
                                    ai.ai_mode = Some(AiMode::Plan(PlanAiMode {
                                        current_plan: finished_plan,
                                        current_action: None,
                                    }));
                                } else {
                                    failed = true;
                                }
                            }
                            GoalResult::Failure => failed = true,
                        }
                    }
                }
            }
            if failed {
                ai.ai_mode = None;
                ai.confusion_ends = Some(game.tick_counter + AI_CONFUSION_LENGTH);
            }
            game.ai_system.ais.insert(ai_id, ai);
        }
    }
    pub fn new(game: &mut Game, game_object_id: GameObjectId) -> AiId {
        let ai_id = game.get_id();
        let ai = Ai {
            game_object_id,

            confusion_ends: None,
            ai_mode: None,
        };
        game.ai_system.ais.insert(ai_id, ai);
        ai_id
    }
    pub fn remove(game: &mut Game, id: AiId) {
        game.ai_system.ais.remove(id);
    }
}
