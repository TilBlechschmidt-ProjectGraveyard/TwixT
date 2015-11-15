extern crate time;
extern crate rustc_serialize;
use rustc_serialize::json;

const BOARD_WIDTH: usize = 24;
const PREALLOC_LINKS: usize = 20;

#[derive(RustcDecodable, RustcEncodable)]
struct Game {
	board: Vec<Vec<u8>>,
	links: Vec<[(u16, u16); 2]>,
	scores: (u8, u8)
}

impl Game {
	fn new() -> Game {
		Game {
			board: vec![vec![0; BOARD_WIDTH]; BOARD_WIDTH],
			links: Vec::with_capacity(PREALLOC_LINKS),
			scores: (0, 0)
		}
	}

	fn next_round(&mut self) {
		// TODO: Do something in here that actually makes sense :D
		self.links.push([(1, 2), (3, 4)]);
		self.links.push([(9, 8), (7, 6)]);
		self.board[6][9] = 1;
	}

	fn run(&mut self) -> (i8, i8) {
		// TODO: If possible remove i as it's not required
		for x in 0..30 { self .next_round()}
		(24, 1) // Return the total scores
	}
}

fn main() {
	let start_time = time::precise_time_ns();

		let mut g = Game::new();
	g.run();

	let time = time::precise_time_ns() - start_time;

		let encoded = json::encode(&g).unwrap();
		println!("{}", encoded);
	println!("The whole calculation took {} nanoseconds", time);
}

