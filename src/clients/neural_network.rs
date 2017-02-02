use structures::*;
use rustc_serialize::json::{EncoderError, DecoderError, self};
use rand::Rng;
use clients::Client;

pub type Float = f64;
pub type Location = (usize, usize);
const PREALLOC_IO: usize = 576;
pub const INPUT_LENGTH: usize = 5761;//1729;
pub const OUTPUT_LENGTH: usize = 576;
const GRADIENT: Float = 1.0;

#[derive(Debug)]
enum Error {
    InputLengthMismatch
}

impl Error {
    fn print(&self) -> String {
        match *self {
            Error::InputLengthMismatch => {
                "Input length mismatched expected length.".to_string()
            }
        }
    }
}

enum Interpolation {
    Sigmoid,
    Linear(Float)
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Neuron {
    pub bias: Float,
    pub weights: Vec<Float>
}

impl Neuron {
    pub fn new(bias: Float) -> Neuron {
        Neuron {
            bias: bias,
            weights: Vec::with_capacity(PREALLOC_IO)
        }
    }

    pub fn new_random<R: Rng>(weight_count: usize, rng: &mut R) -> Neuron {
        Neuron {
            bias: rng.next_f64() * 2.0 - 1.0,//TODO: Set some random bias //rng.next_f64()*weight_count as f64,
            weights: (0..weight_count).map(|_| rng.next_f64() * 2.0 - 1.0).collect()
        }
    }

    fn calculate(&self, inputs: &Vec<Float>, interpolation: Interpolation) -> Float {
        let weighted_sum = inputs.iter().zip(self.weights.iter())
            .fold(self.bias, |weighted_sum, (input, weight)| {
                weighted_sum + input * weight
            });
        match interpolation {
            Interpolation::Sigmoid => {
                1f64 / (1f64 + (-weighted_sum).exp())
            },
            Interpolation::Linear(gradient) => {
                gradient * weighted_sum
            }
        }
    }
}

#[derive(Clone)]
pub struct NeuralNetwork {
    hidden: Vec<Vec<Neuron>>
}

impl NeuralNetwork {
    pub fn new_random<R: Rng>(assembly: Vec<usize>, rng: &mut R) -> NeuralNetwork {
        NeuralNetwork {
            hidden: assembly.windows(2).map(|window| {
                (0..window[1]).map(|_| Neuron::new_random(window[0], rng)).collect()
            }).collect()
        }
    }

    pub fn calculate(&self, input: Vec<Float>, gradient: Float) -> Result<Vec<Float>, Error> {
        if !(input.len() == self.hidden[0][0].weights.len()) {
            Err(Error::InputLengthMismatch)
        } else {
            Ok(self.hidden.iter().enumerate().fold(input, |input, (index, layer)| {
                layer.iter().map(|neuron| {
                    neuron.calculate(&input,
                        if index == (self.hidden.len()-1) {
                            Interpolation::Linear(gradient)
                        } else {
                            Interpolation::Sigmoid
                        })
                }).collect()
            }))
        }
    }

    pub fn mutate<R: Rng>(&mut self, amount: f32, strength: f32, rng: &mut R) {
        for layer in self.hidden.iter_mut() {
            for neuron in layer.iter_mut() {
                if rng.next_f32() < amount {
                    neuron.bias += (rng.next_f64() * 2.0 - 1.0) * strength as f64;
                    if neuron.bias > 1.0 {
                        neuron.bias = 1.0;
                    } else if neuron.bias < -1.0 {
                        neuron.bias = -1.0;
                    }
                }
                for weight in neuron.weights.iter_mut() {
                    if rng.next_f32() < amount {
                        *weight += (rng.next_f64() * 2.0 - 1.0) * strength as f64;
                        if *weight > 1.0 {
                            *weight = 1.0;
                        } else if *weight < -1.0 {
                            *weight = -1.0;
                        }
                    }
                }
            }
        }
    }

    pub fn encode(&self) -> Result<String, EncoderError> { json::encode(&self.hidden) }

    pub fn decode(input: String) -> Result<NeuralNetwork, DecoderError> {
        json::decode(&input).map(|hidden_layer|
            NeuralNetwork {
                hidden: hidden_layer
            }
        )
    }
}

impl Client for NeuralNetwork {
    fn run(&self, b: &Board, l: &Links, player: u8) -> Move {
        let mut b_in = Vec::with_capacity(BOARD_WIDTH*BOARD_WIDTH*2);
        let mut l_in = [ [ [0;8]; BOARD_WIDTH]; BOARD_WIDTH];
        //TODO: Multiple links in one spot arent represented with this!!!
        //for link in l { l_in[link[0]][link[1]] = 0.125 * (get_link_direction(*link) as Float) - 0.0625 };

        //TODO: Both ends should contain the link! And this made everything slow as hell
        for link in l { l_in[link[0]][link[1]][get_link_direction(*link)] = 1; }
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_WIDTH {
                let x = b[x][y];
                if x == 0 { b_in.push(0.0); b_in.push(0.0); }
                else if x == 1 { b_in.push(1.0); b_in.push(0.0); }
                else if x == 2 { b_in.push(0.0); b_in.push(1.0); }
                else if x == 3 { b_in.push(1.0); b_in.push(1.0); }
            }
        }

        let mut input: Vec<Float> = Vec::new();
        input.extend(b_in.into_iter());
        //input.extend(flatten_array(l_in).into_iter());
        input.extend(flatten_links(l_in).into_iter());
        input.push(player as Float);

        //println!("{}", input.len());

        let result: usize = match self.calculate(input, GRADIENT) {
            Ok(res) => {
                res.iter().enumerate().fold((0, res[0]), |(last_index, last), (index, current)| {
                    if last < *current { (index, *current) } else { (last_index, last) }
                }).0
            },
            Err(err) => {
                println!("{}", err.print());
                1
            }
        };

        //TODO: If the best selected move is invalid then choose the second best one selected by the NN

        Move { x: result%24, y: result/24 }
    }
}


// TODO: Move this into a helper crate (dupe from server.rs)
fn get_link_direction(link: [usize; 4]) -> usize {
    let direction;
    let start = [link[0], link[1]];
    let end = [link[2], link[3]];
    if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 1
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1] + 2 == end[1]) {
        direction = 2
    } else if (start[0] + 1 == end[0]) && (start[1] + 2 == end[1]) {
        direction = 3
    } else if (start[0] + 2 == end[0]) && (start[1] + 1 == end[1]) {
        direction = 4
    } else if (start[0] + 2 == end[0]) && (start[1].saturating_sub(1) == end[1]) {
        direction = 5
    } else if (start[0] + 1 == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 6
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 7
    } else if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 8
    } else {
        direction = 0
    }
    direction
}

fn flatten_links(arr: [[[usize; 8]; BOARD_WIDTH]; BOARD_WIDTH]) -> [Float; BOARD_WIDTH*BOARD_WIDTH*8] {
    let mut i = 0;
    let mut out = [0.0; BOARD_WIDTH*BOARD_WIDTH*8];
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_WIDTH {
            for z in 0..8 {
                out[i] = arr[x][y][z] as Float;
                i = i + 1;
            }
        }
    }
    out
}
