extern crate rustc_serialize;
extern crate term_painter;
use self::term_painter::*;
use self::term_painter::Color::*;

mod interface;

pub const BOARD_WIDTH: usize = 24;

type Field = usize;
type Link = [(usize, usize); 8];
pub struct Move { pub x: usize, pub y: usize }
pub type Board = [[Field; BOARD_WIDTH]; BOARD_WIDTH];
pub type Links = [[Link; BOARD_WIDTH]; BOARD_WIDTH];
pub type Player = fn(&Board, &Links) -> Move;
pub type Clients = (Player, Player);


pub struct Game {
	board: Board,
	links: Links,
	clients: Clients,
	pub scores: (u8, u8)
}

impl Game {
	pub fn new(p1: Player, p2: Player, swamps: bool) -> Game {
		let mut g = Game {
			board: [[0; BOARD_WIDTH]; BOARD_WIDTH],
			links: [[[(0, 0); 8]; BOARD_WIDTH]; BOARD_WIDTH],
			clients: (p1, p2),
			scores: (0, 0)
		};
		if swamps { g.generate_swamps() }
		g
	}

	fn generate_swamps(&mut self) {
		self.board[0][0] = 3;
		self.board[0][BOARD_WIDTH-1] = 3;
		self.board[BOARD_WIDTH-1][0] = 3;
		self.board[BOARD_WIDTH-1][BOARD_WIDTH-1] = 3;
	}

	#[cfg(feature = "internal_server")]
	fn execute_move(&mut self, mv: Move, player_id: usize) {
		interface::server::execute_move(&mut self.board, mv, player_id);
	}

	#[cfg(feature = "xml")]
	fn execute_move(&mut self, mv: Move, player_id: usize) {
		interface::xml::execute_move(&mut self.board, mv, player_id);
	}

	//#[cfg(feature = "internal_server")]
	pub fn run(&mut self) -> (i8, i8) {
		for _ in 0..30 {
			let mv = self.clients.0(&self.board, &self.links);
			self.execute_move(mv, 0);

			let mv = self.clients.1(&self.board, &self.links);
			self.execute_move(mv, 1);
		}
		(24, 5) // Return the total scores
	}

	pub fn print_board(&self) {
		//I'm pretty sure this is possible in one loop
		for x in 0..BOARD_WIDTH {
			print!("{}", Black.bg(Black).paint("|"));
			for y in 0..BOARD_WIDTH {
				let cur_loc = self.board[x][y];
				if (y == 0 || y == 23 || x == 0 || x == 23) && cur_loc == 0 {
					if x == 0 || x == 23 { print!("{}", BrightBlue.bg(White).paint(" — ")) }
					else { print!("{}", Red.bg(White).paint(" | ")) }
				} else if cur_loc == 1 {
					print!("{}", Red.bg(Red).paint(" 1 "));
				} else if cur_loc == 2 {
					print!("{}", Blue.bg(Blue).paint(" 2 "));
				} else if cur_loc == 3 {
					print!("{}", Green.bg(Green).paint(" # "));
				} else {
					print!(" . ")
				}
			}
			println!("{}", Black.bg(Black).paint("|"));
		}
	}
}

// pub struct Server {
// 	games: Vec<Game>,
// 	clients: Vec<(Player, Player)>
// }

// impl Server {
// 	pub fn new(amount_of_games: usize) -> Server {
// 		Server {
// 			games: Vec::with_capacity(amount_of_games),
// 			clients: Vec::with_capacity(amount_of_games)
// 		}
// 	}
//
// 	pub fn add_game(&mut self, p1: Player, p2: Player) -> usize {
// 		self.games.push(Game::new(true));
// 		self.clients.push((p1, p2));
// 		self.games.len()-1
// 	}
//
// 	pub fn run(&mut self, game_id: &usize) -> (i8, i8) {
// 		let clients = &self.clients[*game_id];
// 		for _ in 0..30 {
// 			self.games[*game_id].execute_player(clients.0, 0);
// 			self.games[*game_id].execute_player(clients.1, 1);
// 		}
// 		(24, 5) // Return the total scores
// 	}
// }
