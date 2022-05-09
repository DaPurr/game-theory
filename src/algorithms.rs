use std::hash;

use crate::{GameState, PureStrategyProfile};

/// Various algorithms for game-theoretical concepts.
pub struct Algorithms;

impl Algorithms {
    /// Return a subgame-perfect NE (SPNE).
    ///
    /// Returns an arbitray SPNE, even if there exist multiple.
    pub fn subgame_perfect_nash_equilibrium<InformationSet, State, Player, Action>(
        root: State,
    ) -> PureStrategyProfile<InformationSet, Action>
    where
        InformationSet: Clone + Eq + hash::Hash,
        State: GameState<InformationSet, Player, Action> + Clone,
        Action: Clone,
    {
        let mut strategy_profile = PureStrategyProfile::default();
        Self::recurse(&root, &mut strategy_profile);

        strategy_profile
    }

    fn recurse<InformationSet, State, Player, Action>(
        state: &State,
        strategy_profile: &mut PureStrategyProfile<InformationSet, Action>,
    ) -> Option<f32>
    where
        InformationSet: Clone + Eq + hash::Hash,
        State: GameState<InformationSet, Player, Action> + Clone,
        Action: Clone,
    {
        let mut best_utility = None;
        for action in state.actions() {
            let successor = state.clone().advance(action.clone());
            let utility = Self::recurse(&successor, strategy_profile);
            match (utility, best_utility) {
                (Some(x), Some(y)) => {
                    if x > y {
                        let information_set = successor.information_set();
                        best_utility = Some(x);
                        strategy_profile.insert(information_set.clone(), action)
                    }
                }
                (Some(x), None) => best_utility = Some(x),
                _ => panic!("successor does not give utility"),
            }
        }

        best_utility
    }
}

// todo: spne always returns 1, perhaps there exists none, perhaps there exist multiple?
