use crate::{
    Action,
    Strategy,
};
/// Will start cooperating. It opponent defects once will alway defect.
pub struct Grudger;

impl Strategy for Grudger {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        if other.iter().any(|x| *x == Action::Defect) {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
    fn name(&self) -> String {
        "Grudger".to_string()
    }
}

/// Will start defecting. If opponent cooperates once it will alway cooperate.
pub struct Reluctant;

impl Strategy for Reluctant {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        if other.iter().any(|x| *x == Action::Cooperate) {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }
    fn name(&self) -> String {
        "Reluctant".to_string()
    }
}
