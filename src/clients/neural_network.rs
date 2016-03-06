#![allow(dead_code)]

extern crate rand;
use std::f64::*;
use std::collections::HashMap;
use structures::*;
use self::rand::Rng;
use clients::Client;

pub type Float = f64;
pub type Location = (usize, usize);
const PREALLOC_IO: usize = 576;
const LINK_LENGTH: usize = 1153; // 24*24 + 24*24 + 1 // Board, Links, PlayerID
pub const OUTPUT_LENGTH: usize = 576;

pub struct Link {
    weight: Float,
    pub value: Float
}

impl Link {
    fn new(weight: Float) -> Link {
        Link {
            weight: weight,
            value: 0.0
        }
    }

    fn set_value(&mut self, value: Float) {
        self.value = value * self.weight;
    }
}

pub struct SigmoidNeuron<'a> {
    bias: Float,
    inputs: Vec<&'a Link>,
    outputs: Vec<Link>,
}

impl<'a> SigmoidNeuron<'a> {
    pub fn new(bias: Float) -> SigmoidNeuron<'a> {
        SigmoidNeuron {
            bias: bias,
            inputs: Vec::with_capacity(PREALLOC_IO),
            outputs: Vec::with_capacity(PREALLOC_IO)
        }
    }

    fn apply(&mut self) {
        // Add the bias to the mix
        let mut out = self.bias;
        // Add all input values to the mix (weight already applied by input struct)
        for input in self.inputs.iter() { out = out + input.value };
        // Apply the sigmoid function to the resulting value
        out = 1.0 / (1.0 + consts::E.powf(out));
        // Send the resulting value to all the outputs
        for output in self.outputs.iter_mut() { output.set_value(out) };
    }

    pub fn add_input(&mut self, input: &'a Link) {
        self.inputs.push(input);
    }

    pub fn add_output(&'a mut self, weight: Float) -> &'a Link {
        let output = Link::new(weight);
        self.outputs.push(output);
        self.outputs.get(self.outputs.len()).unwrap()
    }
}

pub struct Output<'a> {
    inputs: Vec<&'a Link>,
    bias: Float,
    gradient: Float,
    value: Float
}

impl<'a> Output<'a> {
    fn new(bias: Float, gradient: Float) -> Output<'a> {
        Output {
            bias: bias,
            inputs: Vec::with_capacity(PREALLOC_IO),
            gradient: gradient,
            value: 0.0
        }
    }

    fn apply(&mut self) -> Float {
        // Add the bias to the mix
        let mut out = self.bias;
        // Add all input values to the mix (weight already applied by input struct)
        for input in self.inputs.iter() { out = out + input.value };
        // Apply a linear function to the resulting value
        out = self.gradient * out;
        // Send the resulting value to all the outputs
        self.value = out;
        out
    }
}

pub struct NeuralNetwork<'b> {
    player: u8,
    inputs: HashMap<usize, Link>,
    hidden: HashMap<usize, HashMap<usize, SigmoidNeuron<'b>>>,
    outputs: HashMap<usize, Output<'b>>
}

impl<'b> NeuralNetwork<'b> {
    pub fn new(player: u8, input_weights: [Float; LINK_LENGTH]) -> NeuralNetwork<'b> {
        let mut inputs: HashMap<usize, Link> = HashMap::with_capacity(LINK_LENGTH);
        for i in 0..LINK_LENGTH { inputs.insert(0, Link::new(input_weights[i])); }
        NeuralNetwork {
            player: player,
            inputs: inputs,
            hidden: HashMap::new(),
            outputs: HashMap::with_capacity(BOARD_WIDTH*BOARD_WIDTH)
        }
    }

    fn apply_layer(layer: &mut Vec<SigmoidNeuron<'b>>) {
        for neuron in layer.iter_mut() { neuron.apply() };
    }

    fn set_inputs(&mut self, input_values: Vec<Float>) {
        for (value, input) in input_values.iter().zip(self.inputs.iter_mut()) {
            input.1.set_value(*value);
        }
    }

    pub fn link_neuron(&'b mut self, start_neuron: Location, end_neuron: Location) -> bool {
        // // Add link for start neuron
        // let maybe_link = self.hidden.get_mut(&start_neuron.0)
        //     .and_then(|origin_layer| origin_layer.get_mut(&start_neuron.1))
        //     .map(|origin_neuron| origin_neuron.add_output(1.0));
        //
        // // Add link for end neuron
        // self.hidden.get_mut(&end_neuron.0)
        //     .and_then(|dest_layer| dest_layer.get_mut(&end_neuron.1))
        //     .and_then(|dest_neuron| {
        //         maybe_link.map(|link| {
        //             dest_neuron.add_input(link);
        //         })
        //     })
        //     .is_some()
        true
    }

    pub fn create_neuron(&mut self, location: Location, bias: Float) {
        if !(self.hidden.contains_key(&location.0)) { self.hidden.insert(location.0, HashMap::new()); }
        self.hidden.get_mut(&location.0).unwrap().insert(location.1, SigmoidNeuron::new(bias));
    }

    pub fn create_output(&mut self, id: usize, bias: Float, gradient: Float) {
        self.outputs.insert(id, Output::new(bias, gradient));
    }
}

impl<'b> Client for NeuralNetwork<'b> {
    fn run(&mut self, b: &Board, l: &Links) -> Move {
        let mut b_in = [[0.0; BOARD_WIDTH]; BOARD_WIDTH];
        let mut l_in = [[0.0; BOARD_WIDTH]; BOARD_WIDTH];
        for link in l { l_in[link[0]][link[1]] = 0.125 * (get_link_direction(*link) as Float) - 0.0625 };
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_WIDTH {
                let x = b[x][y];
                b_in[x][y] =
                    if x == 0 { 0.125 }
                    else if x == 1 { 0.375 }
                    else if x == 2 { 0.625 }
                    else if x == 3 { 0.875 }
                    else { 0.0 };
            }
        }

        let mut input: Vec<Float> = Vec::new();
        input.extend(flatten_array(b_in).into_iter());
        input.extend(flatten_array(l_in).into_iter());
        input.push(self.player as Float);
        self.set_inputs(input);

        let x = rand::thread_rng().gen_range(1, 23);
        let y = rand::thread_rng().gen_range(8, 12);
        Move { x: x, y: y }
    }
}


// TODO: Move this into a helper crate (dupe from server.rs)
fn get_link_direction(link: [usize; 4]) -> usize {
    let direction;
    let start = [link[0], link[1]];
    let end = [link[2], link[3]];
    if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 0
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1] + 2 == end[1]) {
        direction = 1
    } else if (start[0] + 1 == end[0]) && (start[1] + 2 == end[1]) {
        direction = 2
    } else if (start[0] + 2 == end[0]) && (start[1] + 1 == end[1]) {
        direction = 3
    } else if (start[0] + 2 == end[0]) && (start[1].saturating_sub(1) == end[1]) {
        direction = 4
    } else if (start[0] + 1 == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 5
    } else if (start[0].saturating_sub(1) == end[0]) && (start[1].saturating_sub(2) == end[1]) {
        direction = 6
    } else if (start[0].saturating_sub(2) == end[0]) && (start[1] + 1 == end[1]) {
        direction = 7
    } else {
        direction = 8
    }
    direction
}

fn flatten_array(arr: [[Float; BOARD_WIDTH]; BOARD_WIDTH]) -> [Float; BOARD_WIDTH*BOARD_WIDTH] {
    let mut i = 0;
    let mut out = [0.0; BOARD_WIDTH*BOARD_WIDTH];
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_WIDTH {
            out[i] = arr[x][y];
            i = i + 1;
        }
    }
    out
}

// fn rotate(matrix: Board) -> Board {
//     let mut ret = [[0; BOARD_WIDTH]; BOARD_WIDTH];
//
//     for i in 0..BOARD_WIDTH {
//         for j in 0..BOARD_WIDTH {
//             ret[i][j] = matrix[BOARD_WIDTH - j - 1][i];
//         }
//     }
//
//     ret
// }
