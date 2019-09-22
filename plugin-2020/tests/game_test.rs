use std::iter::once;
use std::collections::HashMap;
use socha_plugin_2020::game::{Board, PlayerColor, Field, Piece, PieceType, BOARD_RADIUS, FIELD_COUNT};
use socha_plugin_2020::util::{AxialCoords, CubeCoords};

macro_rules! assert_unordered_eq {
	($a:expr, $b:expr) => {
		assert_eq!(
			$a.into_iter().collect::<::std::collections::HashSet<_>>(),
			$b.into_iter().collect::<::std::collections::HashSet<_>>()
		)
	};
}

#[test]
pub fn test_empty_ascii_hex_grid() {
	let ascii_hex = r#"    /\  /\    
   /  \/  \   
   |   |   |  
  /\  /\  /\  
 /  \/  \/  \ 
 |   |   |   |
 \  /\  /\  / 
  \/  \/  \/  
   |   |   |  
   \  /\  /   
    \/  \/    "#;
	let board = Board::from_ascii_hex_grid(ascii_hex).expect("Board could not be converted");
	assert_eq!(board.fields().count(), 7);
	assert!(!board.has_pieces());
}

#[test]
pub fn test_filled_ascii_hex_grid() {
	let ascii_hex = r#"    /\  /\  /\
   /  \/  \ / \
   |   |   |   | 
  /\  /\  /\  /\
 /  \/  \/  \/  \
 |   |BR |AB |AB |
 \  /\  /\  /\  /
  \/  \/  \/  \/
   |   |GR |   |
  /\  /\  /\  /\
 /  \/  \/  \/  \
 |   |   |   |   |
 \  /\  /\  /\  /
  \/  \/  \/  \/
   |   |   |   | 
   \  /\  /\  /
    \/  \/  \/"#;
	let board = Board::from_ascii_hex_grid(ascii_hex).expect("Board could not be converted");
	assert_eq!(board.fields().count(), 17);
	assert!(board.has_pieces());
	assert_unordered_eq!(board.fields_owned_by(PlayerColor::Red).map(|(c, f)| (c, f.clone())), vec![
		(AxialCoords::new(0, 0), Field::new(once(Piece {
			piece_type: PieceType::Grasshopper,
			owner: PlayerColor::Red
		}), false)),
		(AxialCoords::new(0, 1), Field::new(once(Piece {
			piece_type: PieceType::Bee,
			owner: PlayerColor::Red
		}), false))
	]);
	assert_unordered_eq!(board.fields_owned_by(PlayerColor::Blue).map(|(c, f)| (c, f.clone())), vec![
		(AxialCoords::new(1, 0), Field::new(once(Piece {
			piece_type: PieceType::Ant,
			owner: PlayerColor::Blue
		}), false)),
		(AxialCoords::new(2, -1), Field::new(once(Piece {
			piece_type: PieceType::Ant,
			owner: PlayerColor::Blue
		}), false))
	]);
}

#[test]
fn test_filling_radius() {
	let board = Board::filling_radius(BOARD_RADIUS, HashMap::new());
	assert_eq!(board.fields().count(), FIELD_COUNT);
	for (coords, _) in board.fields() {
		let cube_coords = CubeCoords::from(coords);
		assert_eq!(cube_coords.x() + cube_coords.y() + cube_coords.z(), 0);
	}
}
