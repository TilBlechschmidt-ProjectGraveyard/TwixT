extern crate time;
extern crate rand;
extern crate rustc_serialize;
mod structures;
mod clients;
use clients::*;
use structures::*;

//const NEURONS: usize = 600;

fn main() {
	let nn = NeuralNetwork::new_random(vec![INPUT_LENGTH, 1000, 893, 839, 720, OUTPUT_LENGTH]);
	for i in 0..100 { println!("{}, {}", i, nn.calculate(vec![i as f64/100.0], 1.0).unwrap()[0]) }

	let mut g = Game::new(SimpleClient::new(0), nn, true);

	let start_time = time::precise_time_ns();
	let scores = g.run();
	let time = (time::precise_time_ns() - start_time) / 1000;

	g.print_board();
	g.print_links();
	println!("The whole calculation took {}μs", time);
	println!("Therefore one round took {}μs", (((time as f64) / 30.0) * 100.0).round() / 100.0);
	println!("Scores are ({}, {})", scores[0], scores[1]);
}
