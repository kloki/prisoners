use crate::{
    Action,
    Strategy,
};
/// Will start Cooperate, Defect, Cooperate, Cooperate.
/// Afterwards, if you ever retaliate with a Defect, it plays like tit for tat.
/// Otherwise, it plays like an exploiter.
pub struct Detective;

impl Strategy for Detective {
    fn next_move(&self, other: &Vec<Action>, _: &Vec<Action>) -> Action {
        match other.len() {
            0 | 2 | 3 => Action::Cooperate,
            1 => Action::Defect,
            _ => {
                if other[2] == Action::Defect {
                    match other.iter().last() {
                        Some(s) => *s,
                        _ => Action::Cooperate,
                    }
                } else {
                    Action::Defect
                }
            }
        }
    }
    fn name(&self) -> String {
        "Detective".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detective_against_tit_for_tat() {
        let input = vec![
            Action::Cooperate,
            Action::Cooperate,
            Action::Defect,
            Action::Cooperate,
        ];
        assert_eq!(Detective.next_move(&input, &vec![]), Action::Cooperate);
    }
    #[test]
    fn test_detective_against_exploiter() {
        let input = vec![
            Action::Defect,
            Action::Defect,
            Action::Defect,
            Action::Defect,
        ];
        assert_eq!(Detective.next_move(&input, &vec![]), Action::Defect);
    }
    #[test]
    fn test_detective_against_naive() {
        let input = vec![
            Action::Cooperate,
            Action::Cooperate,
            Action::Cooperate,
            Action::Cooperate,
        ];
        assert_eq!(Detective.next_move(&input, &vec![]), Action::Defect);
    }
}
