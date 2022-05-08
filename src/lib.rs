//! Crate providing game-theoretical concepts.
#[cfg(test)]
mod test;

/// Trait denoting a game's state.
///
/// Used to progress a game until it is finished. Can also be seen as a node
/// in a game tree.
pub trait GameState<Action, InformationSet, Player> {
    /// Returns whether the current state represents a terminal node.
    fn is_terminal(&self) -> bool;

    /// Perform ```action``` to advance the game state.
    fn advance(self, action: Action) -> Self;

    /// Obtain this node's action set.
    ///
    /// It is the user's responsibility to ensure all game nodes in an information set
    /// produce the same action set.
    fn actions(&self) -> Box<dyn Iterator<Item = Action>>;

    /// Retrieve the information set that this node is partitioned in.
    ///
    /// The user should ensure all characteristics of an information set hold.
    fn information_set(&self) -> &InformationSet;

    /// Retrieve the player associated to this node.
    fn player(&self) -> &Player;
}

/// Represents the history ```h_t``` up until the current time point ```t```.
#[derive(Debug)]
pub struct History<T> {
    actions: Vec<T>,
}

/// Structure containing algorithms for various equilibrium concepts.
pub struct Algorithms;

/// Strategy associated to a certain player.
pub struct Strategy;

impl<T> Default for History<T> {
    fn default() -> Self {
        History { actions: vec![] }
    }
}
