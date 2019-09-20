use socha_client_base::plugin::{SCPlugin, HasPlayerColor};
use crate::game;

/// The concrete plugin for the "Hive" game.
#[derive(Debug)]
pub struct SCPlugin2020;

impl SCPlugin for SCPlugin2020 {
	type PlayerColor = game::PlayerColor;
	type Player = game::Player;
	type GameState = game::GameState;
	type Move = game::Move;
	
	fn protocol_game_type<'a>() -> &'a str { "swc_2020_hive" }
}

impl HasPlayerColor for game::GameState {
	type PlayerColor = game::PlayerColor;
	
	fn player_color(&self) -> game::PlayerColor { self.current_player_color }
}
