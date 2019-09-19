use socha_client_base::plugin::SCPlugin;
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
