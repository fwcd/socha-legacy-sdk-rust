use socha_client_base::plugin::{SCPlugin, HasPlayerColor, HasTurn};
use crate::game;

#[derive(Debug)]
pub struct SCPlugin2021;

impl SCPlugin for SCPlugin2021 {
    type PlayerColor = game::Team;
    type Player = game::Player;
    type GameState = game::GameState;
    type Move = game::Move;

    fn protocol_game_type<'a>() -> &'a str { "swc_2021_blokus" }
}

impl HasPlayerColor for game::GameState {
    type PlayerColor = game::Team;

    fn player_color(&self) -> Self::PlayerColor { self.ordered_colors[self.current_color_index as usize].team() }
}

impl HasTurn for game::GameState {
    fn turn(&self) -> u32 { self.turn }
}
