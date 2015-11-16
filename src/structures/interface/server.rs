use structures::*;

fn is_inside_enemy_base(loc: &Move, player: &usize) -> bool {
    if *player == 0 && (loc.x == 0 || loc.x == BOARD_WIDTH - 1) { return true }
    else if *player == 1 && (loc.y == 0 || loc.y == BOARD_WIDTH - 1) { return true }
    false
}

pub fn execute_move(board: &mut Board, mv: Move, player_id: usize) {
	if board[mv.x][mv.y] != 0 {
		println!("INVALID MOVE BY PLAYER {} - Field already occupied. ({}, {})", player_id, mv.x, mv.y);
	} else if board[mv.x][mv.y] == 3 {
		println!("INVALID MOVE BY PLAYER {}  - Collision with swamp. ({}, {})", player_id, mv.x, mv.y);
	} else if is_inside_enemy_base(&mv, &player_id) {
		println!("INVALID MOVE BY PLAYER {}  - Inside enemy base. ({}, {})", player_id, mv.x, mv.y);
	} else {
		board[mv.x][mv.y] = player_id + 1
	}
}
