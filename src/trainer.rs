#![allow(dead_code)]
use std::cmp::Ordering;
use clients::*;
use structures::*;
use rand::Rng;

//const ASSEMBLY: &'static[usize] = &[INPUT_LENGTH, 1000, 893, 839, 720, OUTPUT_LENGTH];

pub struct TrainerConfig {
    /// Amount of clients that one given generation consists of
    generation_size: usize,
    /// Amount of survivors that get mutated to the next generation
    survivor_count: usize,
    /// Amount of best clients that set the standard for the next generation
    score_references: usize,
    /// Amount of games a client takes place in to calculate the avg. score
    games_per_client: usize,
    /// Percentage how many parameters should be changed while mutating
    mutation_amount: f32,
    /// Value by what the weights should be updated
    mutation_strength: f32,
    /// Structure of the NeuralNetwork
    structure: Vec<usize>
}

impl TrainerConfig {
    pub fn build_new_trainer<R: Rng>(&self, rng: &mut R) -> Trainer {
        Trainer {
            survivor_count: self.survivor_count,
            games_per_client: self.games_per_client,
            mutation_amount: self.mutation_amount,
            mutation_strength: self.mutation_strength,
            score_references:
                TrainerConfig::generate_random_generation(&self.structure, self.score_references, rng),
            current_generation:
                TrainerConfig::generate_random_generation(&self.structure, self.generation_size, rng)
        }
    }

    pub fn new() -> TrainerConfig {
        TrainerConfig {
            generation_size: 60,
            survivor_count: 20,
            score_references: 5,
            games_per_client: 10,
            mutation_amount: 0.1,
            mutation_strength: 0.1,
            structure: vec![INPUT_LENGTH, 600, 300, 150, 100, 100, 100, OUTPUT_LENGTH]
        }
        // TrainerConfig {
        //     generation_size: 30,
        //     survivor_count: 5,
        //     score_references: 2,
        //     games_per_client: 4,
        //     mutation_amount: 0.30,
        //     mutation_strength: 0.1529,
        //     structure: vec![INPUT_LENGTH, 500, 300, OUTPUT_LENGTH]
        // }
    }

    fn generate_random_generation<R: Rng>(structure: &Vec<usize>, size: usize, rng: &mut R) -> Vec<NeuralNetwork> {
        (0..size).map(|_| {
            NeuralNetwork::new_random(structure.clone(), rng)
        }).collect()
    }
}

pub struct Trainer {
    survivor_count: usize,
    games_per_client: usize,
    mutation_amount: f32,
    mutation_strength: f32,
    score_references: Vec<NeuralNetwork>,
    current_generation: Vec<NeuralNetwork>,
}

impl Trainer {
    pub fn step<R: Rng>(&mut self, rng: &mut R) {

        let games_per_ref = self.games_per_client / self.score_references.len();

        //println!("DEBUG: Calculating average scores");

        let mut avg_scores: Vec<(usize, f32)> = self.current_generation.iter().enumerate().map(|(index, contestant)| {
            let score_sum =
                self.score_references.iter().flat_map(|reference| {
                    (0..games_per_ref).map(|_| {
                        //println!("DEBUG: Starting new game");
                        //TODO: Use the same game field for the same generation.
                        //Game::new_random(contestant, reference, rng).run()[0];
                        let sc = SimpleClient::new(1);
                        let mut g = Game::new_random(contestant, &sc, rng);

                        let scores = g.run();//[0]
                        g.print_board();
                        //println!("{}", contestant.encode().unwrap());
                        scores[0]
                    }).collect::<Vec<_>>()
                }).fold(0, |acc, score| score + acc );
            let avg_score = score_sum as f32 / (games_per_ref * self.score_references.len()) as f32;
            (index, avg_score)
        }).collect();

        //println!("DEBUG: Sorting average scores");

        avg_scores.sort_by(|a, b| if a.1 < b.1 {Ordering::Less} else {Ordering::Greater});

        //println!("DEBUG: Fetching new score refs");

        let new_score_refs = avg_scores
            .iter().take(self.score_references.len())
            .map(|&(index, _)| self.current_generation[index].clone())
            .collect();

        let generation_size = self.current_generation.len();
        //println!("{} - {} / {}", generation_size, self.survivor_count, self.survivor_count);
        let mutation_per_survivor = (generation_size - self.survivor_count) / self.survivor_count;

        //println!("DEBUG: Mutating survivors");

        let new_generation = avg_scores.iter().take(self.survivor_count).flat_map(|&(index, _)| {
            let survivor = self.current_generation[index].clone();
            let mut mutations = (0..mutation_per_survivor).map(|_| {
                let mut mutation = survivor.clone();
                mutation.mutate(self.mutation_amount, self.mutation_strength, rng);
                mutation
            }).collect::<Vec<_>>();

            mutations.push(survivor);
            mutations
        }).collect::<Vec<_>>();

        if !(generation_size == new_generation.len()) {
            println!("GENERATION SIZE CHANGED {} ---> {}", self.current_generation.len(), new_generation.len());
        }

        println!("Best average score of generation x is: {}", avg_scores[0].1);

        self.current_generation = new_generation;
        self.score_references = new_score_refs;
    }
}

// fn run_game(nn1: String, nn2: String) -> [u8; 2] {
//     let mut g = Game::new(NeuralNetwork::decode(nn1).unwrap(), NeuralNetwork::decode(nn2).unwrap(), true);
//     let scores = g.run();
//
//     g.print_board();
//     scores
// }
