use socha_plugin_2020::game::Board;

#[test]
pub fn test_ascii_hex_to_board() {
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
	assert!(!board.has_pieces());
	assert_eq!(board.fields().count(), 7);
}
