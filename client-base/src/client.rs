use std::str::FromStr;
use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::{info, debug, error};
use xml::reader::{XmlEvent, EventReader};
use crate::xml_node::{XmlNode, FromXmlNode};
use crate::util::SCResult;
use crate::error::SCError;
use crate::plugin::SCPlugin;
use crate::protocol::{Joined, Left, Room, Data, GameResult};

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
	/// The plugin defining which types are
	/// representing various parts of the game.
	type Plugin: SCPlugin;

	/// Invoked whenever the game state updates.
	fn on_update_state(&mut self, state: &<Self::Plugin as SCPlugin>::GameState) {}
	
	/// Invoked when the game ends.
	fn on_game_end(&mut self, result: GameResult<Self::Plugin>) {}
	
	/// Invoked when the welcome message is received
	/// with the player's color.
	fn on_welcome_message(&mut self, color: &<Self::Plugin as SCPlugin>::PlayerColor) {}
	
	/// Requests a move from the delegate. This method
	/// should implement the "main" game logic.
	fn request_move(&mut self, state: &<Self::Plugin as SCPlugin>::GameState, my_color: <Self::Plugin as SCPlugin>::PlayerColor) -> <Self::Plugin as SCPlugin>::Move;
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> where D: SCClientDelegate {
	delegate: D,
	debug_mode: bool,
	my_color: Option<<D::Plugin as SCPlugin>::PlayerColor>,
	game_state: Option<<D::Plugin as SCPlugin>::GameState>
}

impl<D> SCClient<D> 
	where
		D: SCClientDelegate,
		<<D as SCClientDelegate>::Plugin as SCPlugin>::GameState: FromXmlNode,
		<<D as SCClientDelegate>::Plugin as SCPlugin>::PlayerColor: FromStr,
		SCError: From<<<<D as SCClientDelegate>::Plugin as SCPlugin>::PlayerColor as FromStr>::Err> {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D, debug_mode: bool) -> Self {
		Self { delegate: delegate, debug_mode: debug_mode, my_color: None, game_state: None }
	}
	
	/// Blocks the thread and begins reading XML messages
	/// from the provided address via TCP.
	pub fn run(self, host: &str, port: u16, reservation: Option<&str>) -> SCResult<()> {
		let address = format!("{}:{}", host, port);
		let mut stream = TcpStream::connect(&address)?;
		info!("Connected to {}", address);
		
		{
			let mut writer = BufWriter::new(&stream);
			writer.write("<protocol>".as_bytes())?;
			
			let join_xml = match reservation {
				Some(res) => format!("<joinPrepared reservationCode=\"{}\" />", res),
				None => "<join gameType=\"swc_2020_hive\" />".to_owned()
			};
			info!("Sending join message {}", join_xml);
			writer.write(join_xml.as_bytes())?;
		}
		
		if self.debug_mode {
			// In debug mode, only the raw XML messages will be output
			io::copy(&mut stream, &mut io::stdout())?;
		} else {
			// In normal mode, begin parsing game messages from the stream
			self.run_game(BufReader::new(stream))?;
		}
		
		Ok(())
	}
	
	/// Blocks the thread and parses/handles game messages
	/// from the provided reader.
	fn run_game<R>(mut self, reader: R) -> SCResult<()> where R: Read {
		let mut xml_parser = EventReader::new(reader);
		
		// Read initial protocol element
		info!("Waiting for initial <protocol>...");
		while match xml_parser.next() {
			Ok(XmlEvent::StartElement { name, .. }) => Some(name),
			_ => None
		}.filter(|n| n.local_name == "protocol").is_none() {}

		loop {
			let node = XmlNode::read_from(&mut xml_parser)?;
			debug!("Got XML node {:#?}", node);
			
			if let Ok(room) = <Room<D::Plugin>>::from_node(&node) {
				// Got room message (the game is running)
				match room.data {
					Data::WelcomeMessage { color } => {
						info!("Got welcome message with color: {:?}", color);
						self.delegate.on_welcome_message(&color);
						self.my_color = Some(color);
					},
					Data::Memento { state } => {
						info!("Got updated game state");
						self.delegate.on_update_state(&state);
						self.game_state = Some(state);
					},
					Data::MoveRequest => {
						info!("Got move request");
						if let Some(ref state) = self.game_state {
							if let Some(my_color) = self.my_color {
								let new_move = self.delegate.request_move(state, my_color);
							} else {
								error!("Can not fulfill move request without a color!");
							}
						} else {
							error!("Can not fulfill move request without a game state!");
						}
					},
					Data::GameResult { result } => {
						info!("Got game result: {:?}", result);
						self.delegate.on_game_end(result);
					}
				}
			} else if let Ok(joined) = Joined::from_node(&node) {
				// Got 'joined' message
				info!("Joined room {}", joined.room_id);
			} else if let Ok(left) = Left::from_node(&node) {
				// Got 'left' message
				info!("Left room {}", left.room_id);
			}
		}
	}
}
