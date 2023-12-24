use rand::Rng;

use crate::{
    Action,
    Strategy,
};
/// Will start cooperating. If the opposing player defects it will also start defecting.
/// Untill the opossing player cooperates again.
pub struct TitForTat;

impl Strategy for TitForTat {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        match other.iter().last() {
            Some(s) => *s,
            _ => Action::Cooperate,
        }
    }
    fn name(&self) -> String {
        "TitForTat".to_string()
    }
}

/// Will start defecting. If the opposing player defects it will also start defecting.
/// Untill the opossing player cooperates again.
pub struct SuspicousTitForTat;
impl Strategy for SuspicousTitForTat {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        match other.iter().last() {
            Some(s) => *s,
            _ => Action::Defect,
        }
    }
    fn name(&self) -> String {
        "SuspicousTFT".to_string()
    }
}

/// Will start cooperating. If the opposing player defects it will also start defecting 90% of the
/// time. Untill the opossing player cooperates again.
pub struct GenerousTitForTat;
impl Strategy for GenerousTitForTat {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        let mut rng = rand::thread_rng();
        match other.iter().last() {
            Some(Action::Defect) => {
                if rng.gen_range(0.0..1.0) < 0.1 {
                    Action::Cooperate
                } else {
                    Action::Defect
                }
            }
            _ => Action::Cooperate,
        }
    }
    fn name(&self) -> String {
        "GenerousTFT".to_string()
    }
}

/// Will start cooperating. If the opposing player defects it will also start defecting
/// Untill the opossing player cooperates again. It sneaks in a defect 10% of the time.
pub struct CheeckyTitForTat;
impl Strategy for CheeckyTitForTat {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        let mut rng = rand::thread_rng();
        match other.iter().last() {
            Some(Action::Cooperate) => {
                if rng.gen_range(0.0..1.0) < 0.1 {
                    Action::Defect
                } else {
                    Action::Cooperate
                }
            }
            Some(Action::Defect) => Action::Defect,
            _ => Action::Cooperate,
        }
    }
    fn name(&self) -> String {
        "CheeckyTFT".to_string()
    }
}
/// Will start cooperating. If the opposing player defects N in a row it
/// will also start defecting. If the the opossing player cooperates it will also cooperate again.
pub struct TitForTatN(pub usize);

impl Strategy for TitForTatN {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        let other_len = other.len();
        if other_len < self.0 {
            return Action::Cooperate;
        }
        if other[(other_len - self.0)..(other_len - 1)]
            .iter()
            .any(|x| *x == Action::Defect)
        {
            return Action::Defect;
        }
        Action::Cooperate
    }
    fn name(&self) -> String {
        format!("TitForTat{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tit_for_tat() {
        assert_eq!(TitForTat.next_move(&vec![], &vec![]), Action::Cooperate);
        let input = vec![Action::Defect, Action::Cooperate];
        assert_eq!(TitForTat.next_move(&input, &vec![]), Action::Cooperate);
        let input2 = vec![Action::Defect, Action::Defect];
        assert_eq!(TitForTat.next_move(&input2, &vec![]), Action::Defect);
    }

    #[test]
    fn test_tit_for_tat_n() {
        let ttn = TitForTatN(2);
        assert_eq!(ttn.next_move(&vec![], &vec![]), Action::Cooperate);
        let mut input = vec![Action::Defect];
        assert_eq!(ttn.next_move(&input, &vec![]), Action::Cooperate);
        input = vec![Action::Defect, Action::Defect];
        assert_eq!(ttn.next_move(&input, &vec![]), Action::Defect);
        input = vec![Action::Defect, Action::Cooperate, Action::Defect];
        assert_eq!(ttn.next_move(&input, &vec![]), Action::Cooperate);
    }
}
