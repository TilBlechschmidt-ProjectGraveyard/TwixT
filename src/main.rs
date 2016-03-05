extern crate time;
extern crate rand;
mod structures;
mod clients;
use clients::*;
use structures::*;
use self::rand::Rng;

const NEURONS: usize = 600;

fn main() {
	let mut weights = [0.0f64; 1153];
	for i in 0..weights.len() {
		weights[i] = rand::thread_rng().next_f64();
	}
	let mut nn = NeuralNetwork::new(1, weights);
	for x in 0..NEURONS { nn.create_neuron((x, 0), 0.0) }
	// for input in nn.inputs.iter_mut() {
	// 	for neuron in nn.layer0.iter() {
	// 		neuron.add_input(input);
	// 	}
	// }
	for i in 0..(BOARD_WIDTH*BOARD_WIDTH) { nn.create_output(i, 0.0, 1.0) }


	let mut g = Game::new(SimpleClient::new(0), NeuralNetwork::new(1, weights), true);

	let start_time = time::precise_time_ns();
	let scores = g.run();
	let time = (time::precise_time_ns() - start_time) / 1000;

	g.print_board();
	g.print_links();
	println!("The whole calculation took {}μs", time);
	println!("Therefore one round took {}μs", (((time as f64) / 30.0) * 100.0).round() / 100.0);
	println!("Scores are ({}, {})", scores[0], scores[1]);
}
