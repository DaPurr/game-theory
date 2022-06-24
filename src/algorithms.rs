use std::{hash, panic};

use crate::{GameState, Outcome, PureStrategyProfile};

/// Various algorithms for game-theoretical concepts.
pub struct Algorithms;

impl Algorithms {
    /// Return a subgame-perfect NE (SPNE).
    ///
    /// Returns an arbitray pure strategy SPNE, even if there exist multiple.
    /// The user has the responsibility to ensure that all information sets are singletons.
    pub fn subgame_perfect_nash_equilibrium<InformationSet, State, Player, Action>(
        root: State,
    ) -> PureStrategyProfile<InformationSet, Action>
    where
        InformationSet: Clone + Eq + hash::Hash,
        State: GameState<InformationSet, Player, Action> + Clone + std::fmt::Debug,
        Action: Clone,
        Player: Eq + hash::Hash,
    {
        if let Some(player) = root.player() {
            let mut strategy_profile = PureStrategyProfile::default();
            let outcome = Self::evaluate(&root, &mut strategy_profile);
            println!("u = {:?}", outcome.utility(player));
            return strategy_profile;
        } else {
            panic!("root does not have player")
        }
    }

    fn evaluate<InformationSet, State, Player, Action>(
        state: &State,
        strategy_profile: &mut PureStrategyProfile<InformationSet, Action>,
    ) -> Outcome<Player>
    where
        InformationSet: Clone + Eq + hash::Hash,
        State: GameState<InformationSet, Player, Action> + Clone + std::fmt::Debug,
        Action: Clone,
        Player: Eq + hash::Hash,
    {
        // endpoint
        if state.is_terminal() {
            return state.outcome();
        }

        // init
        let mut best_outcome_option: Option<Outcome<Player>> = None;
        let player = state
            .player()
            .expect("state with successor should have player associated to it");

        // max utility over action set
        let information_set = state.information_set();
        for action in state.actions() {
            let successor = state
                .clone()
                .advance(action.clone())
                .expect("all non-terminal states should have successors");

            // we need to update action in state's information set, not successor's
            match (
                &best_outcome_option,
                Self::evaluate(&successor, strategy_profile),
            ) {
                // action is better, so update
                (Some(best_outcome), outcome) => {
                    let u_max = best_outcome.utility(player);
                    let u = outcome.utility(player);
                    if u > u_max {
                        best_outcome_option = Some(outcome);
                        strategy_profile.insert(information_set.clone(), action);
                    }
                }
                // no action as reference, so set one
                (None, outcome) => {
                    best_outcome_option = Some(outcome);
                    strategy_profile.insert(information_set.clone(), action);
                }
            }
        }
        best_outcome_option.expect("internal error")
    }
}

// todo: spne always returns 1 spne, perhaps there exists none, perhaps there exist multiple?
