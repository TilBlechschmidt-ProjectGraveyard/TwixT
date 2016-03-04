use structures::*;

fn is_inside_enemy_base(loc: &Move, player: &usize) -> bool {
    if  (*player == 0 && (loc.x == 0 || loc.x == BOARD_WIDTH - 1)) ||
        (*player == 1 && (loc.y == 0 || loc.y == BOARD_WIDTH - 1)) {
            return true
        };
    false
}

fn get_field(board: &mut Board, x: usize, y: usize) -> usize {
    match board.get(x) {
        Some(x) => {
            match x.get(y) {
                Some(y) => {
                    return *y;
                }
                None => { return 404 }
            }
        }
        None => { return 404 }
    }
}

fn is_link(xi: i32, yi: i32, direction: usize, links: &Links) -> bool {
    if xi < 0 || yi < 0 { return false };
    let x: usize = xi as usize;
    let y: usize = yi as usize;
    let x2;
    let y2;
    if direction == 0 {
        x2 = x - 2;
        y2 = y + 1;
    } else if direction == 1 {
        x2 = x - 1;
        y2 = y + 2;
    } else if direction == 2 {
        x2 = x + 1;
        y2 = y + 2;
    } else if direction == 3 {
        x2 = x + 2;
        y2 = y + 1;
    } else if direction == 4 {
        x2 = x + 2;
        y2 = y - 1;
    } else if direction == 5 {
        x2 = x + 1;
        y2 = y - 2;
    } else if direction == 6 {
        x2 = x - 1;
        y2 = y - 2;
    } else if direction == 7 {
        x2 = x - 2;
        y2 = y - 1;
    } else {
        x2 = x;
        y2 = y;
    }
    for link in links { if link[0] == x && link[1] == y && link[2] == x2 && link[3] == y2 { return true } }
    return false
}

fn check_link_possibility(start: [usize; 2], end: [usize; 2], links: &Links) -> bool {
    let direction;
    let mut valid = true;
    // TODO: Move this into a helper function (dupe from neural_network.rs)
    if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 0
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1] + 2 == end[1]) {
        direction = 1
    } else if (start[0] + 1 == end[0]) && (start[1] + 2 == end[1]) {
        direction = 2
    } else if (start[0] + 2 == end[0]) && (start[1] + 1 == end[1]) {
        direction = 3
    } else if (start[0] + 2 == end[0]) && (start[1].saturating_sub(1) == end[1]) {
        direction = 4
    } else if (start[0] + 1 == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 5
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 6
    } else if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 7
    } else {
        direction = 8
    }

    let start: [i32; 2] = [start[0] as i32, start[1] as i32];

    if direction == 0 {
        valid = if is_link((start[0]    ), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 3), (start[1]    ), 3, links) { false } else { valid };
    } else if direction == 1 {
        valid = if is_link((start[0] - 1), (start[1] + 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0]    ), (start[1] - 1), 0, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] - 1), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1] - 1), 3, links) { false } else { valid };
    } else if direction == 2 {
        valid = if is_link((start[0] + 1), (start[1] + 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0]    ), (start[1] - 1), 3, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] - 1), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1] - 1), 0, links) { false } else { valid };
    } else if direction == 3 {
        valid = if is_link((start[0]    ), (start[1] + 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] + 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1] + 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 3), (start[1]    ), 0, links) { false } else { valid };
    } else if direction == 4 {       // TODO: Link directions 4-7 need to be tested and potentially fixed
        valid = if is_link((start[0]    ), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 3), (start[1]    ), 0, links) { false } else { valid };
    } else if direction == 5 {
        valid = if is_link((start[0] + 1), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0]    ), (start[1] + 1), 3, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] + 1), 0, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1] + 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] + 2), (start[1] + 1), 0, links) { false } else { valid };
    } else if direction == 6 {
        valid = if is_link((start[0] - 1), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] + 1), (start[1]    ), 0, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0]    ), (start[1] + 1), 0, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] + 1), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] + 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1] + 1), 3, links) { false } else { valid };
    } else if direction == 7 {
        valid = if is_link((start[0]    ), (start[1] - 1), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1] - 1), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 1), (start[1]    ), 1, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 3, links) { false } else { valid };
        valid = if is_link((start[0] - 2), (start[1]    ), 2, links) { false } else { valid };
        valid = if is_link((start[0] - 3), (start[1]    ), 3, links) { false } else { valid };
    } else {
        valid = false;
    }
    //if !valid { println!("LINKS CROSSING - Preventing creation.") };
    valid
}

fn set_link_if_possible(links: &mut Links, board: &mut Board, loc: Move, player: usize) -> bool {
    let length = links.len();
    //println!("{} {}", loc.x, loc.y);
    if loc.x > 0 &&                 get_field(board, loc.x - 1, loc.y + 2) == player && check_link_possibility([loc.x, loc.y], [loc.x - 1, loc.y + 2], links) { links.push([loc.x, loc.y, loc.x - 1, loc.y + 2]) }
    if                              get_field(board, loc.x + 1, loc.y + 2) == player && check_link_possibility([loc.x, loc.y], [loc.x + 1, loc.y + 2], links) { links.push([loc.x, loc.y, loc.x + 1, loc.y + 2]) }
    if                              get_field(board, loc.x + 2, loc.y + 1) == player && check_link_possibility([loc.x, loc.y], [loc.x + 2, loc.y + 1], links) { links.push([loc.x, loc.y, loc.x + 2, loc.y + 1]) }
    if loc.y > 0 &&                 get_field(board, loc.x + 2, loc.y - 1) == player && check_link_possibility([loc.x, loc.y], [loc.x + 2, loc.y - 1], links) { links.push([loc.x, loc.y, loc.x + 2, loc.y - 1]) }
    if loc.x > 0 && loc.y > 1 &&    get_field(board, loc.x - 1, loc.y - 2) == player && check_link_possibility([loc.x, loc.y], [loc.x - 1, loc.y - 2], links) { links.push([loc.x, loc.y, loc.x - 1, loc.y - 2]) }
    if loc.y > 1 &&                 get_field(board, loc.x + 1, loc.y - 2) == player && check_link_possibility([loc.x, loc.y], [loc.x + 1, loc.y - 2], links) { links.push([loc.x, loc.y, loc.x + 1, loc.y - 2]) }
    if loc.x > 1 && loc.y > 0 &&    get_field(board, loc.x - 2, loc.y - 1) == player && check_link_possibility([loc.x, loc.y], [loc.x - 2, loc.y - 1], links) { links.push([loc.x, loc.y, loc.x - 2, loc.y - 1]) }
    if loc.x > 1 &&                 get_field(board, loc.x - 2, loc.y + 1) == player && check_link_possibility([loc.x, loc.y], [loc.x - 2, loc.y + 1], links) { links.push([loc.x, loc.y, loc.x - 2, loc.y + 1]) }
    if links.len() != length { true } else { false }
}

pub fn execute_move(board: &mut Board, links: &mut Links, mv: Move, player_id: usize) -> bool {
	// if board[mv.x][mv.y] == 3 {
    //     println!("INVALID MOVE BY PLAYER {}  - Collision with swamp. ({}, {})", player_id, mv.x, mv.y);
	// } else if board[mv.x][mv.y] != 0 {
    //     println!("INVALID MOVE BY PLAYER {} - Field already occupied. ({}, {})", player_id, mv.x, mv.y);
	// } else if is_inside_enemy_base(&mv, &player_id) {
	//     println!("INVALID MOVE BY PLAYER {}  - Inside enemy base. ({}, {})", player_id, mv.x, mv.y);
	// } else {
	// 	   board[mv.x][mv.y] = player_id + 1;
    //     if set_link_if_possible(links, board, mv, player_id+1) { return true };
	// }
    if board[mv.x][mv.y] == 0 && !(is_inside_enemy_base(&mv, &player_id)) {
        board[mv.x][mv.y] = player_id + 1;
        if set_link_if_possible(links, board, mv, player_id+1) { return true };
    }
    false
}
