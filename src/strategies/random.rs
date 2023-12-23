use rand::{
    distributions::{
        Distribution,
        Standard,
    },
    Rng,
};

use crate::{
    Action,
    Strategy,
};
impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=1) {
            // rand 0.8
            0 => Action::Cooperate,
            _ => Action::Defect,
        }
    }
}

/// Will return a random action
pub struct Random;

impl Strategy for Random {
    fn next_move(&self, _: &Vec<Action>, _: &Vec<Action>) -> Action {
        rand::random()
    }
    fn name(&self) -> String {
        "Random".to_string()
    }
}

/// Will start randomly after that it will switching between the options.
pub struct FlipFlop;

impl Strategy for FlipFlop {
    fn next_move(&self, _: &Vec<Action>, own_moves: &Vec<Action>) -> Action {
        match own_moves.iter().last() {
            Some(s) => s.flip(),
            _ => rand::random(),
        }
    }
    fn name(&self) -> String {
        "FlipFlop".to_string()
    }
}
