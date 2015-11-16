extern crate time;
mod structures;
mod clients;
use clients::simple_client;
use structures::*;


fn main() {

	//TODO: Make this a borrowed pointer to the player instead of a passed function
	let mut g = Game::new(simple_client::run, simple_client::run);

	let start_time = time::precise_time_ns();
	g.run();
	let time = (time::precise_time_ns() - start_time) / 1000;

	//let encoded = json::encode(&g.board).unwrap();
	//println!("{}", encoded);
	g.print_board();
	println!("The whole calculation took {}μs", time);
	println!("Therefore one round took {}μs", time/30);
	println!("Scores are p1: {}, p2: {}", g.scores.0, g.scores.1);
}
