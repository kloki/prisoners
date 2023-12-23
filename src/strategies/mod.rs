pub use base::{
    Exploiter,
    Naive,
};
pub use grudger::{
    Grudger,
    Reluctant,
};
pub use random::{
    FlipFlop,
    Random,
};
pub use titfortats::{
    TitForTat,
    TitForTatN,
};
mod base;
mod grudger;
mod random;
mod titfortats;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Cooperate,
    Defect,
}

impl Action {
    pub fn flip(&self) -> Action {
        match self {
            Action::Cooperate => Action::Defect,
            Action::Defect => Action::Cooperate,
        }
    }
}

pub trait Strategy {
    fn next_move(&self, other_actions: &Vec<Action>, own_actions: &Vec<Action>) -> Action;
    fn name(&self) -> String;
}
