use crate::{
    Action,
    Strategy,
};
/// Will always cooperate
pub struct Naive;

impl Strategy for Naive {
    fn next_move(&self, _: &Vec<Action>, _: &Vec<Action>) -> Action {
        Action::Cooperate
    }
    fn name(&self) -> String {
        "Naive".to_string()
    }
}

/// Will always defect
pub struct Exploiter;

impl Strategy for Exploiter {
    fn next_move(&self, _: &Vec<Action>, _: &Vec<Action>) -> Action {
        Action::Defect
    }
    fn name(&self) -> String {
        "Exploiter".to_string()
    }
}
