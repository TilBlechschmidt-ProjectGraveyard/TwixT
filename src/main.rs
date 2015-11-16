extern crate time;
mod structures;
mod clients;
use clients::simple_client;
use structures::*;

fn main() {

	//TODO: Make this a borrowed pointer to the player instead of a passed function
	let mut g = Game::new(simple_client::run, simple_client::run, true);

	let start_time = time::precise_time_ns();
	let scores = g.run();
	let time = (time::precise_time_ns() - start_time) / 1000;

	g.print_board();
	println!("The whole calculation took {}μs", time);
	println!("Therefore one round took {}μs", time/30);
	println!("Scores are ({}, {})", scores.0, scores.1);
}
