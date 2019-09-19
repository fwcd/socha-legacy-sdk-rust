use std::fmt::Debug;
use crate::util::HasOpponent;

/// An "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin: Debug {
	type PlayerColor: Copy + Debug + Eq + HasOpponent;
	type Player<'a>: Clone + Debug + Eq;
	type GameState<'a>: Clone + Debug + Eq;
	type Move: Clone + Debug + Eq;
}
