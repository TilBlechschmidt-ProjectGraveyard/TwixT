const BOARD_WIDTH: usize = 24;

type Field = u8;
type Link = [(u8, u8); 8];
pub type Board = [[Field; BOARD_WIDTH]; BOARD_WIDTH];
pub type Links = [[Link; BOARD_WIDTH]; BOARD_WIDTH];
pub type Player = fn(&Board, &Links) -> (u8, u8);


//#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
	board: Board,
	links: Links, //[[Link; BOARD_WIDTH]; BOARD_WIDTH],
	players: [Player; 2],
	pub scores: (u8, u8)
}

impl Game {
	pub fn new(p1: Player, p2: Player) -> Game {
		Game {
			board: [[0; BOARD_WIDTH]; BOARD_WIDTH],
			links: [[[(0, 0); 8]; BOARD_WIDTH]; BOARD_WIDTH],
			players: [p1, p2],
			scores: (0, 0)
		}
	}

	fn next_round(&mut self) {
		// TODO: Do something in here that actually makes sense :D
		self.links[0][0][0] = (3, 4);
		self.links[1][0][1] = (6, 7);
		self.board[6][9] = 1;
	}

	pub fn run(&mut self) -> (i8, i8) {
		for _ in 0..30 { self .next_round()}
		(24, 5) // Return the total scores
	}
}
