extern crate rustc_serialize;
extern crate term_painter;
extern crate rand;
use self::rand::Rng;
use self::term_painter::*;
use self::term_painter::Color::*;

mod interface;
use super::clients::*;

pub const BOARD_WIDTH: usize = 24;
pub const FIELD_SWAMP: usize = 3;

type Field = usize;
type Link = [usize; 4]; //[(usize, usize); 8];
pub struct Move { pub x: usize, pub y: usize }
pub type Board = [[Field; BOARD_WIDTH]; BOARD_WIDTH];
pub type Links = Vec<Link>;//[[Link; BOARD_WIDTH]; BOARD_WIDTH];
pub type Player = SimpleClient;
pub type Clients = (Player, Player);


pub struct Game<A: Client, B: Client> {
	board: Board,
	links: Links,
	clients: (A, B),
	pub scores: (u8, u8)
}

impl<A: Client, B: Client> Game<A, B> {
	pub fn new(p1: A, p2: B, swamps: bool) -> Game<A, B> {
		let mut g = Game {
			board: [[0; BOARD_WIDTH]; BOARD_WIDTH],
			links: Vec::with_capacity(24), //[[[0; 4]; BOARD_WIDTH]; BOARD_WIDTH],
			clients: (p1, p2),
			scores: (0, 0)
		};
		if swamps { g.generate_swamps() }
		g
	}

	fn is_rect_empty(&self, loc: (Field, Field), width: Field, height: Field) -> bool {
		for x_offset in 0..width {
			for y_offset in 0..height {
				if self.board[loc.0 + x_offset][loc.1 + y_offset] != 0 { return false };
			}
		}
		true
	}

	fn generate_swamp(&mut self, width: Field, height: Field) {
		let mut swamp_location;
		let mut rng = rand::thread_rng();
		let mut counter = 0;
		loop {
			swamp_location = (rng.gen_range(1, BOARD_WIDTH - width), rng.gen_range(1, BOARD_WIDTH - height));
			if self.is_rect_empty(swamp_location, width, height) { break };
			if counter > 50 { println!("COULDN'T GENERATE SWAMP - Skipping."); return };
			counter = counter + 1;
		}
		for x_offset in 0..width {
			for y_offset in 0..height {
				self.board[swamp_location.0 + x_offset][swamp_location.1 + y_offset] = FIELD_SWAMP;
			}
		}
	}

	fn generate_corner_swamps(&mut self) {
		self.board[0][0] = FIELD_SWAMP;
		self.board[0][BOARD_WIDTH-1] = FIELD_SWAMP;
		self.board[BOARD_WIDTH-1][0] = FIELD_SWAMP;
		self.board[BOARD_WIDTH-1][BOARD_WIDTH-1] = FIELD_SWAMP;
	}

	fn generate_swamps(&mut self) {
		self.generate_corner_swamps();
		self.generate_swamp(3, 3);
		self.generate_swamp(2, 2);
		self.generate_swamp(2, 2);
		self.generate_swamp(1, 1);
	}

	fn score_branch(&self, link: Link, player: usize, done: &mut Links) -> Links {
		let mut endpoints = Vec::new();
		let done_tmp = done.clone();
		let sub_branches = self.links.iter().filter(|&x|
				((x[0] == link[0] && x[1] == link[1]) || (x[2] == link[2] && x[3] == link[3]))
			&& 	!(*x == link)
			&&  !(done_tmp.contains(x))
			&&	self.board[link[0]][link[1]] == player+1
		);

		let mut endpoint = true;
		for sub_branch in sub_branches {
			done.push(*sub_branch);
			endpoints.extend(self.score_branch(*sub_branch, player, done).into_iter());
			endpoint = false;
		}
		if endpoint { return vec![link] } else { endpoints }
	}

	fn recalculate_score(&mut self, player: usize, scores: &mut [u8; 2]) {
		for link in self.links.iter() {
			if self.board[link[0]][link[1]] == player+1 {
				//println!("BRANCHING @ {} {} -> {} {}", link[0], link[1], link[2], link[3]);
				let endpoints = self.score_branch(*link, player, &mut vec![*link]);
				for endpoint in endpoints.iter() {
					let d1; let d2; let d3;
					if player == 0 {
						// Scores for the Y axis
						d1 = endpoint[1] as i16 - link[1] as i16;
						d2 = endpoint[3] as i16 - link[1] as i16;
						d3 = endpoint[1] as i16 - link[3] as i16;
					} else {
						// Scores for the X axis
						d1 = endpoint[0] as i16 - link[0] as i16;
						d2 = endpoint[2] as i16 - link[0] as i16;
						d3 = endpoint[0] as i16 - link[2] as i16;
					}
					let new_score =
							 if d1 > d2 && d1 > d3 { d1 }
						else if d2 > d1 && d2 > d3 { d2 }
						else if d3 > d2 && d3 > d1 { d3 }
						else { 0 };
					if new_score > (scores[player] as i16) { scores[player] = new_score as u8 };
				}
			}
		}
	}

	#[cfg(feature = "internal_server")]
	fn execute_move(&mut self, mv: Move, player_id: usize) -> bool {
		interface::server::execute_move(&mut self.board, &mut self.links, mv, player_id)
	}

	#[cfg(feature = "xml")]
	fn execute_move(&mut self, mv: Move, player_id: usize) {
		interface::xml::execute_move(&mut self.board, mv, player_id);
	}

	//#[cfg(feature = "internal_server")]
	pub fn run(&mut self) -> [u8; 2] {
		let mut scores = [0, 0];
		for _ in 0..30 {
			let mv = self.clients.0.run(&self.board, &self.links);
			if self.execute_move(mv, 0) { self.recalculate_score(0, &mut scores) };

			let mv = self.clients.1.run(&self.board, &self.links);
			if self.execute_move(mv, 1) { self.recalculate_score(1, &mut scores) };
		}
		scores
	}

	pub fn print_board(&self) {
		//I'm pretty sure this is possible in one loop
		for y in 0..BOARD_WIDTH {
			print!("{}", Black.bg(Black).paint("|"));
			for x in 0..BOARD_WIDTH {
				let cur_loc = self.board[x][y];
				if (y == 0 || y == 23 || x == 0 || x == 23) && cur_loc == 0 {
					if x == 0 || x == 23 { print!("{}", BrightBlue.bg(White).paint(" | ")) }
					else { print!("{}", Red.bg(White).paint(" — ")) }
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
	pub fn print_links(&self) {
		for link in self.links.iter() {
			println!("{} {} -> {} {}", link[0], link[1], link[2], link[3]);
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
