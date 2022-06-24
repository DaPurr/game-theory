use std::{collections::HashMap, panic};

use crate::{algorithms::Algorithms, GameState, Outcome};

#[test]
fn test_spne() {
    let root = UltimatumState::default();
    let spne = Algorithms::subgame_perfect_nash_equilibrium(root);

    println!("SPNE:");
    for i in spne.information_sets() {
        if let Some(action) = spne.action(i) {
            println!("information set {}: action = {:?}", i, action)
        }
    }
}

#[derive(Clone, Debug)]
enum UltimatumAction {
    Fair,
    Unfair,
    Accept,
    Reject,
}

#[derive(Clone, Debug)]
struct UltimatumState {
    action1: Option<UltimatumAction>,
    action2: Option<UltimatumAction>,
}

impl Default for UltimatumState {
    fn default() -> Self {
        UltimatumState {
            action1: None,
            action2: None,
        }
    }
}

impl GameState<usize, usize, UltimatumAction> for UltimatumState {
    fn advance(mut self, action: UltimatumAction) -> Option<Self> {
        match (&self.action1, &self.action2) {
            (None, None) => self.action1 = Some(action),
            (Some(_), None) => self.action2 = Some(action),
            (Some(_), Some(_)) => return None,
            _ => panic!("illegal state: {:?}", self),
        }
        Some(self)
    }

    fn actions(&self) -> Box<dyn Iterator<Item = UltimatumAction>> {
        match (&self.action1, &self.action2) {
            (None, None) => Box::new([UltimatumAction::Fair, UltimatumAction::Unfair].into_iter()),
            (Some(_), None) => {
                Box::new([UltimatumAction::Accept, UltimatumAction::Reject].into_iter())
            }
            (Some(_), Some(_)) => Box::new([].into_iter()),
            _ => panic!("illegal state: {:?}", self),
        }
    }

    fn information_set(&self) -> &usize {
        match (&self.action1, &self.action2) {
            (None, None) => &0,
            (Some(UltimatumAction::Fair), None) => &1,
            (Some(UltimatumAction::Unfair), None) => &2,
            // terminals
            _ => &3,
        }
    }

    fn player(&self) -> Option<&usize> {
        match (&self.action1, &self.action2) {
            (None, None) => Some(&0),
            (Some(_), None) => Some(&1),
            (Some(_), Some(_)) => None,
            _ => panic!("invalid state: {:?}", self),
        }
    }

    fn utility(&self, player: &usize) -> Option<f32> {
        match (&self.action1, &self.action2) {
            (Some(UltimatumAction::Fair), Some(UltimatumAction::Accept)) => Some(5.),
            (Some(UltimatumAction::Fair), Some(UltimatumAction::Reject)) => Some(0.),
            (Some(UltimatumAction::Unfair), Some(UltimatumAction::Accept)) => {
                if player == &0 {
                    Some(8.)
                } else if player == &1 {
                    Some(2.)
                } else {
                    panic!("invalid state: {:?}", self)
                }
            }
            (Some(UltimatumAction::Unfair), Some(UltimatumAction::Reject)) => Some(0.),
            _ => None,
        }
    }

    fn outcome(&self) -> crate::Outcome<usize> {
        let mut map = HashMap::new();
        match (&self.action1, &self.action2) {
            (Some(UltimatumAction::Fair), Some(UltimatumAction::Accept)) => {
                map.insert(0, 5.);
                map.insert(1, 5.);
            }
            (Some(UltimatumAction::Fair), Some(UltimatumAction::Reject)) => {
                map.insert(0, 0.);
                map.insert(1, 0.);
            }
            (Some(UltimatumAction::Unfair), Some(UltimatumAction::Accept)) => {
                map.insert(0, 8.);
                map.insert(1, 2.);
            }
            (Some(UltimatumAction::Unfair), Some(UltimatumAction::Reject)) => {
                map.insert(0, 0.);
                map.insert(1, 0.);
            }
            _ => panic!("invalid state"),
        }

        Outcome { map }
    }
}
