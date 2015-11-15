extern crate rustc_serialize;
use rustc_serialize::json;

const BOARD_WIDTH: usize = 24;
const PREALLOC_LINKS: usize = 20;

#[derive(RustcDecodable, RustcEncodable)]
struct Game {
board: Vec<Vec<i32>>,
links: Vec<[(i32, i32); 2]>
}

impl Game {
fn new() -> Game {
	Game {
	board: vec![vec![0; BOARD_WIDTH]; BOARD_WIDTH],
	links: Vec::with_capacity(PREALLOC_LINKS)
	}
}
}

fn main() {
	let mut g = Game::new();
	g.links.push([(1, 2), (3, 4)]);
	g.links.push([(9, 8), (7, 6)]);
	g.board[6][9] = 1;
	println!("Set pin at 6, 9 and created two links: ");

	let encoded = json::encode(&g).unwrap();
	println!("{}", encoded);
}

