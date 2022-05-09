//! Crate providing game-theoretical concepts.

use core::panic;
use std::{collections::HashMap, marker::PhantomData};

use petgraph::{
    data::DataMap,
    graph::{DiGraph, GraphIndex, NodeIndex},
    EdgeDirection,
};

pub mod algorithms;
#[cfg(test)]
mod test;

/// Trait denoting a game's state.
///
/// Used to progress a game until it is finished. Can also be seen as a node
/// in a game tree.
pub trait GameState<InformationSet, Player, Action> {
    /// Returns whether the current state represents a terminal node.
    ///
    /// A state is terminal if and only if [GameState::player] returns ```None```.
    fn is_terminal(&self) -> bool {
        self.player().is_none()
    }

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
    /// For example, all nodes in an information set should return the same active player
    /// and the same action set.
    fn information_set(&self) -> &InformationSet;

    /// Retrieve the player associated to this node.
    ///
    /// Only a terminal state should return ```None```.
    fn player(&self) -> Option<&Player>;

    /// Return the utility for ```player``` for ending up in this node.
    ///
    /// Usually only terminal nodes are associated with a utility mapping, but other
    /// nodes are also allowed to have one.
    fn utility(&self, player: &Player) -> Option<f32>;
}

/// Represents the history ```h_t``` up until the current time point ```t```.
#[derive(Debug)]
pub struct History<T> {
    actions: Vec<T>,
}

/// Strategy associated to a certain player.
pub struct Strategy<InformationSet, Action> {
    map: HashMap<InformationSet, ActionDistribution<Action>>,
}

pub struct GameTree<InformationSet, State, Player, Action> {
    tree: DiGraph<State, Action>,
    root: NodeIndex,

    phantom1: PhantomData<InformationSet>,
    phantom2: PhantomData<Player>,
}

struct ActionDistribution<Action> {
    map: HashMap<Action, f32>,
}

pub struct PureStrategyProfile<InformationSet, Action> {
    map: HashMap<InformationSet, Action>,
}

impl<InformationSet, Action> Default for PureStrategyProfile<InformationSet, Action> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<InformationSet, Action> PureStrategyProfile<InformationSet, Action> {
    pub fn insert(&mut self, information_set: InformationSet, action: Action)
    where
        InformationSet: Eq + std::hash::Hash,
    {
        self.map.insert(information_set, action);
    }
}

impl<T> Default for History<T> {
    fn default() -> Self {
        History { actions: vec![] }
    }
}

impl<Action> ActionDistribution<Action> {
    pub fn sample<T: rand::Rng>(&self, rng: &mut T) -> &Action {
        // get random value
        let r: f32 = rng.gen();

        // sample
        let mut sum = 0.;
        for (action, probability) in &self.map {
            sum += probability;
            if r <= sum {
                return action;
            }
        }
        panic!("Mapping over action space does not constitute a probability distribution");
    }

    pub fn weight(&self, action: &Action) -> Option<f32> {
        todo!()
    }
}

impl<InformationSet, State, Player, Action> GameTree<InformationSet, State, Player, Action> {
    pub fn from_root(root: State) -> Self {
        todo!()
    }

    pub fn root(&self) -> &State {
        let idx = self.root;
        self.tree.node_weight(idx).unwrap()
    }

    pub fn terminal_nodes(&self) -> impl Iterator<Item = &State> {
        let mut terminal_nodes = vec![];
        for idx in self.tree.node_indices() {
            let outgoing = self.tree.neighbors_directed(idx, EdgeDirection::Outgoing);
            if outgoing.count() == 0 {
                terminal_nodes.push(self.tree.node_weight(idx).unwrap())
            }
        }

        terminal_nodes.into_iter()
    }

    pub fn predecessors(&self, state: NodeIndex) -> impl Iterator + '_ {
        self.tree.neighbors_directed(state, EdgeDirection::Incoming)
    }

    pub fn node_weight(&self, idx: NodeIndex) -> Option<&State> {
        self.tree.node_weight(idx)
    }
}
