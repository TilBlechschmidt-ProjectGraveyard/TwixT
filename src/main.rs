extern crate time;
extern crate rustc_serialize;
mod structures;
mod clients;
use clients::simple_client;
use structures::*;
//use rustc_serialize::json;


fn main() {
	let start_time = time::precise_time_ns();

	//TODO: Make this a borrowed pointer to the player instead of a passed function
	let mut g = Game::new(simple_client::run, simple_client::run);
	g.run();

	let time = time::precise_time_ns() - start_time;

	//let encoded = json::encode(&g).unwrap();
	//println!("{}", encoded);
	println!("The whole calculation took {} nanoseconds", time);
	println!("Scores are p1: {}, p2: {}", g.scores.0, g.scores.1);
}
