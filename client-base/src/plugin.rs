/// An "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin {
	type PlayerColor;
	type Player;
	type GameState;
	type Move;
}
