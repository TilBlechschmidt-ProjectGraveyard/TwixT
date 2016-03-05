#![allow(dead_code)]

extern crate rand;
use std::f64::*;
use structures::*;
use self::rand::Rng;
use clients::Client;

pub type Float = f64;
const PREALLOC_IO: usize = 576;
const INPUT_LENGTH: usize = 1153; // 24*24 + 24*24 + 1 // Board, Links, PlayerID

pub struct Input {
    weight: Float,
    pub value: Float
}

impl Input {
    fn new(weight: Float) -> Input {
        Input {
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
    inputs: Vec<&'a Input>,
    outputs: Vec<Input>
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

    pub fn add_input(&mut self, input: &'a Input) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: Input) {
        self.outputs.push(output);
    }
}

pub struct Output<'a> {
    inputs: Vec<&'a Input>,
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
    pub inputs: Vec<Input>,
    pub layer0: Vec<SigmoidNeuron<'b>>,
    //pub layer1: Vec<SigmoidNeuron<'b>>,
    pub outputs: Vec<Output<'b>>
}

impl<'b> NeuralNetwork<'b> {
    pub fn new(player: u8, input_weights: [Float; INPUT_LENGTH]) -> NeuralNetwork<'b> {
        let mut inputs = Vec::with_capacity(INPUT_LENGTH);
        for i in 0..INPUT_LENGTH { inputs.push(Input::new(input_weights[i])) };
        NeuralNetwork {
            player: player,
            inputs: inputs,
            layer0: Vec::new(),
            //layer1: Vec::new(),
            outputs: Vec::with_capacity(BOARD_WIDTH*BOARD_WIDTH)
        }
    }

    fn apply_layer(layer: &mut Vec<SigmoidNeuron<'b>>) {
        for neuron in layer.iter_mut() { neuron.apply() };
    }

    fn set_inputs(&mut self, input_values: Vec<Float>) {
        for (value, input) in input_values.iter().zip(self.inputs.iter_mut()) {
            input.set_value(*value);
        }
    }

    pub fn create_neuron(&mut self, bias: Float) {
        self.layer0.push(SigmoidNeuron::new(bias));
    }

    pub fn create_output(&mut self, bias: Float, gradient: Float) {
        self.outputs.push(Output::new(bias, gradient));
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
