pub use base::{
    Exploiter,
    Naive,
};
pub use detective::Detective;
pub use grudger::{
    Grudger,
    Reluctant,
};
use rand::Rng;
pub use random::{
    FlipFlop,
    Random,
};
pub use titfortats::{
    SuspicousTitForTat,
    TitForTat,
    TitForTatN,
};
mod base;
mod detective;
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

    pub fn noise(&self, noise_ratio: f64) -> Action {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0f64..1f64);
        if roll < noise_ratio {
            self.flip()
        } else {
            *self
        }
    }
}

pub trait Strategy {
    fn next_move(&self, other_actions: &Vec<Action>, own_actions: &Vec<Action>) -> Action;
    fn name(&self) -> String;
}
