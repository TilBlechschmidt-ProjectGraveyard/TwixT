use structures::*;

pub fn execute_move(board: &mut Board, mv: Move, player_id: usize) {
	board[mv.x][mv.y] = player_id + 1
}
