use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::{info, debug, warn, error};
use xml::reader::{XmlEvent as XmlReadEvent, EventReader};
use xml::writer::{EventWriter};
use crate::xml_node::{XmlNode, FromXmlNode};
use crate::util::SCResult;
use crate::plugin::{SCPlugin, HasPlayerColor};
use crate::protocol::{Joined, Left, Room, Data, GameResult};

/// A handler that implements the game player's
/// behavior, usually employing some custom move
/// selection strategy.
pub trait SCClientDelegate {
	/// The plugin defining which types are
	/// representing various parts of the game.
	type Plugin: SCPlugin;

	/// Invoked whenever the game state updates.
	fn on_update_state(&mut self, _state: &<Self::Plugin as SCPlugin>::GameState) {}
	
	/// Invoked when the game ends.
	fn on_game_end(&mut self, _result: GameResult<Self::Plugin>) {}
	
	/// Invoked when the welcome message is received
	/// with the player's color.
	fn on_welcome_message(&mut self, _color: &<Self::Plugin as SCPlugin>::PlayerColor) {}
	
	/// Requests a move from the delegate. This method
	/// should implement the "main" game logic.
	fn request_move(&mut self, state: &<Self::Plugin as SCPlugin>::GameState, my_color: <Self::Plugin as SCPlugin>::PlayerColor) -> <Self::Plugin as SCPlugin>::Move;
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> where D: SCClientDelegate {
	delegate: D,
	debug_mode: bool,
	game_state: Option<<D::Plugin as SCPlugin>::GameState>
}

impl<D> SCClient<D> where D: SCClientDelegate {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D, debug_mode: bool) -> Self {
		Self { delegate: delegate, debug_mode: debug_mode, game_state: None }
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
				None => format!("<join gameType=\"{}\" />", D::Plugin::protocol_game_type())
			};
			info!("Sending join message {}", join_xml);
			writer.write(join_xml.as_bytes())?;
		}
		
		if self.debug_mode {
			// In debug mode, only the raw XML messages will be output
			io::copy(&mut stream, &mut io::stdout())?;
		} else {
			// In normal mode, begin parsing game messages from the stream
			let reader = BufReader::new(stream.try_clone()?);
			let writer = BufWriter::new(stream);
			self.run_game(reader, writer)?;
		}
		
		Ok(())
	}
	
	/// Blocks the thread and parses/handles game messages
	/// from the provided reader.
	fn run_game<R, W>(mut self, reader: R, writer: W) -> SCResult<()> where R: Read, W: Write {
		let mut xml_reader = EventReader::new(reader);
		let mut xml_writer = EventWriter::new(writer);
		
		// Read initial protocol element
		info!("Waiting for initial <protocol>...");
		while match xml_reader.next() {
			Ok(XmlReadEvent::StartElement { name, .. }) => Some(name),
			_ => None
		}.filter(|n| n.local_name == "protocol").is_none() {}

		loop {
			let node = XmlNode::read_from(&mut xml_reader)?;
			debug!("Got XML node {:#?}", node);
			
			match node.name() {
				// Try parsing as room message (the game is running)
				"room" => match <Room<D::Plugin>>::from_node(&node) {
					Ok(room) => match room.data {
						Data::WelcomeMessage { color } => {
							info!("Got welcome message with color: {:?}", color);
							self.delegate.on_welcome_message(&color);
						},
						Data::Memento { state } => {
							info!("Got updated game state");
							self.delegate.on_update_state(&state);
							self.game_state = Some(state);
						},
						Data::MoveRequest => {
							info!("Got move request");
							if let Some(ref state) = self.game_state {
								let new_move = self.delegate.request_move(state, state.player_color());
								let move_node = <SCResult<XmlNode>>::from(Room::<D::Plugin> {
									room_id: room.room_id,
									data: Data::Move(new_move)
								})?;
								debug!("Sending move {:#?}", move_node);
								move_node.write_to(&mut xml_writer)?;
							} else {
								error!("Cannot fulfill move request without a game state!");
							}
						},
						Data::GameResult(result) => {
							info!("Got game result: {:?}", result);
							self.delegate.on_game_end(result);
						},
						_ => warn!("Could not handle room data: {:?}", room.data)
					},
					Err(e) => error!("Could not parse node as room: {:?}", e)
				},

				// Try parsing as 'joined' message
				"joined" => match Joined::from_node(&node) {
					Ok(joined) => info!("Joined room {}", joined.room_id),
					Err(e) => error!("Could not parse node as 'joined': {:?}", e)
				},

				// Try parsing as 'left' message
				"left" => match Left::from_node(&node) {
					Ok(left) => info!("Left room {}", left.room_id),
					Err(e) => error!("Could not parse node as 'left': {:?}", e)
				},
				
				_ => warn!("Unrecognized message: <{}>", node.name())
			}
		}
	}
}
