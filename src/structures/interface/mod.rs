#[cfg(feature = "internal_server")]
pub mod server;

#[cfg(feature = "xml")]
pub mod xml;



#[cfg(test)]
#[cfg(feature = "internal_server")]
mod internal_server {
    use structures::*;
    use super::*;

	#[test]
	fn exec_moves() { // Check if the normal placing on free spots works
		let mut board = [[0; BOARD_WIDTH]; BOARD_WIDTH];
		assert_eq!(0, board[0][1]);
		server::execute_move(&mut board, Move { x: 1, y: 1 }, 0);
		assert_eq!(1, board[1][1]);
	}

    #[test]
    fn enemy_bases() { // Check if it's smart enough to recognize enemy bases.
        let mut board = [[0; BOARD_WIDTH]; BOARD_WIDTH];
        assert_eq!(0, board[0][1]);
        server::execute_move(&mut board, Move { x: 0, y: 1 }, 0);
        assert_eq!(0, board[0][1]);
        server::execute_move(&mut board, Move { x: 0, y: 1 }, 1);
        assert_eq!(2, board[0][1]);
    }

    #[test]
    fn blocked_field() { // Check if it's smart enough to recognize that a field is blocked
        let mut board = [[0; BOARD_WIDTH]; BOARD_WIDTH];
        assert_eq!(0, board[1][1]);
        server::execute_move(&mut board, Move { x: 1, y: 1 }, 0);
        assert_eq!(1, board[1][1]);
        server::execute_move(&mut board, Move { x: 1, y: 1 }, 1);
        assert_eq!(1, board[1][1]);
    }

    #[test]
    #[ignore]
    fn link_creation() {
        // TODO: Check if links get created
    }
}

#[cfg(test)]
#[cfg(feature = "xml")]
mod xml_server {
    use structures::*;
    use super::*;
    
    #[test]
    fn exec_moves() { // Check if it's dumb enough to just executes our moves without thinking.
		let mut board = [[0; BOARD_WIDTH]; BOARD_WIDTH];
		for x in 0..BOARD_WIDTH {
			for y in 0..BOARD_WIDTH {
		        assert_eq!(0, board[x][y]);
				xml::execute_move(&mut board, Move { x: x, y: y }, 0);
				assert_eq!(1, board[x][y]);
			}
		}
    }
}
