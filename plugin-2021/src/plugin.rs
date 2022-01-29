use socha_client_base::plugin::{SCPlugin, HasTeam, HasTurn};
use crate::game;

#[derive(Debug)]
pub struct SCPlugin2021;

impl SCPlugin for SCPlugin2021 {
    type Team = game::Team;
    type Player = game::Player;
    type GameState = game::GameState;
    type Move = game::Move;

    fn protocol_game_type<'a>() -> &'a str { "swc_2021_blokus" }
}

impl HasTeam for game::GameState {
    type Team = game::Team;

    fn team(&self) -> Self::Team { self.current_team() }
}

impl HasTurn for game::GameState {
    fn turn(&self) -> u32 { self.turn }
}
