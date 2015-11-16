extern crate rustc_serialize;
extern crate term_painter;
use self::term_painter::*;
use self::term_painter::Color::*;

pub const BOARD_WIDTH: usize = 24;

type Field = usize;
type Link = [(usize, usize); 8];
pub struct Move { pub x: usize, pub y: usize }
pub type Board = [[Field; BOARD_WIDTH]; BOARD_WIDTH];
pub type Links = [[Link; BOARD_WIDTH]; BOARD_WIDTH];
pub type Player = fn(&Board, &Links) -> Move;


pub struct Game {
	board: Board,
	links: Links,
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

	fn execute_player(&mut self, id: usize) {
		let mv = self.players[id](&self.board, &self.links);
		self.board[mv.x][mv.y] = id + 1;
	}

	fn next_round(&mut self) {
		self.execute_player(0);
		self.execute_player(1);
	}

	pub fn print_board(&self) {
		//I'm pretty sure this is possible in one loop
		for x in 0..BOARD_WIDTH {
			print!("|");
			for y in 0..BOARD_WIDTH {
				let cur_loc = self.board[x][y];
				if (y == 0 || y == 23 || x == 0 || x == 23) && cur_loc == 0 {
					if x == 0 || x == 23 { print!("{}", BrightBlue.bg(White).paint(" 0 ")) }
					else { print!("{}", Red.bg(White).paint(" 0 ")) }
				} else if cur_loc == 1 {
					print!("{}", Red.bg(Red).paint(" 1 "));
				} else if cur_loc == 2 {
					print!("{}", Blue.bg(Blue).paint(" 2 "));
				} else if cur_loc == 3 {
					print!("{}", Green.bg(Green).paint(" # "));
				} else {
					print!(" - ")
				}
			}
			println!("|");
		}
	}

	pub fn run(&mut self) -> (i8, i8) {
		for _ in 0..30 { self .next_round()}
		(24, 5) // Return the total scores
	}
}
