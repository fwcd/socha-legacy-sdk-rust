use std::net::TcpStream;
use std::io::{self, BufWriter, BufReader, Read, Write};
use log::info;
use xml::reader::{XmlEvent, EventReader};
use crate::xml_node::XmlNode;
use crate::util::SCResult;
use crate::plugin::SCPlugin;
use crate::protocol::*;

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
	fn on_game_end(&mut self, result: GameResult<<Self::Plugin as SCPlugin>::Player>) {}
	
	/// Invoked when the welcome message is received
	/// with the player's color.
	fn on_welcome_message(&mut self, color: &<Self::Plugin as SCPlugin>::PlayerColor) {}
	
	/// Requests a move from the delegate. This method
	/// should implement the "main" game logic.
	fn request_move(&mut self, state: &<Self::Plugin as SCPlugin>::GameState, me: &<Self::Plugin as SCPlugin>::Player, opponent: &<Self::Plugin as SCPlugin>::Player) -> <Self::Plugin as SCPlugin>::Move;
}

/// The client which handles XML requests, manages
/// the game state and invokes the delegate.
pub struct SCClient<D> {
	delegate: D,
	debug_mode: bool
}

impl<D> SCClient<D> {
	/// Creates a new client using the specified delegate.
	pub fn new(delegate: D, debug_mode: bool) -> Self {
		Self { delegate: delegate, debug_mode: debug_mode }
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
	fn run_game<R>(self, reader: R) -> SCResult<()> where R: Read {
		let mut xml_parser = EventReader::new(reader);
		
		// Read initial protocol element
		info!("Waiting for initial <protocol>...");
		while match xml_parser.next() {
			Ok(XmlEvent::StartElement { name, .. }) => Some(name),
			_ => None
		}.filter(|n| n.local_name == "protocol").is_none() {}

		loop {
			let node = XmlNode::read_from(&mut xml_parser)?;
			info!("Got XML node {:#?}", node);
		}
	}
}
