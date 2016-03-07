#![allow(dead_code)]
use std::cmp::Ordering;
use clients::*;
use structures::*;
use rand::Rng;
use std::thread;
use pbr::ProgressBar;
//use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

const CPU_CORES: usize = 4;

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
        println!("Generating random networks . . .");
        let mut pb = ProgressBar::new(self.score_references + self.generation_size);
        let t = Trainer {
            survivor_count: self.survivor_count,
            games_per_client: self.games_per_client,
            mutation_amount: self.mutation_amount,
            mutation_strength: self.mutation_strength,
            score_references:
                TrainerConfig::generate_random_generation(&self.structure, self.score_references, rng, &mut pb),
            current_generation:
                TrainerConfig::generate_random_generation(&self.structure, self.generation_size, rng, &mut pb)
        };
        t
    }

    pub fn new() -> TrainerConfig {
        TrainerConfig {
            generation_size: 60,
            survivor_count: 20,
            score_references: 5,
            games_per_client: 10,
            mutation_amount: 0.3,
            mutation_strength: 0.1529,
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

    fn generate_random_generation<R: Rng>(structure: &Vec<usize>, size: usize, rng: &mut R, p: &mut ProgressBar) -> Vec<NeuralNetwork> {
        (0..size).map(|_| {
            p.inc();
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
    pub fn step<R: Rng>(&mut self, rng: &mut R) {// -> NeuralNetwork {

        let games_per_ref = self.games_per_client / self.score_references.len();

        // let mut games = Vec::new();
        // for (index, contestant) in self.current_generation.iter().enumerate() {
        //     games.push((index, contestant.clone(), self.score_references.clone()));
        //     // for reference in self.score_references.iter() {
        //     //     games.push((index, contestant.clone(), reference.clone()));
        //     // }
        // }
        // let mut progress = ProgressBar::new(self.current_generation.len()*self.score_references.len()*games_per_ref);
        // let mut pblock = Arc::new(Mutex::new(progress));
        // let threads: Vec<_> = games.into_iter().map(move |game| {
        //     let pb = pblock.clone();
        //     thread::spawn(move || {
        //         //println!("Started new thread!");
        //         let index = game.0;
        //         let contestant = game.1.clone();
        //         let score_references = game.2.clone();
        //         let score_sum = score_references.into_iter().flat_map(move |reference| {
        //             (0..games_per_ref).map(|game_id| {
        //                 //TODO: Use the same game field for the same generation.
        //                 let mut g;
        //                 if game_id%2 == 0 { // Change sides for every second game
        //                     g = Game::new_random(&contestant, &reference);
        //                 } else {
        //                     g = Game::new_random(&reference, &contestant);
        //                 }
        //                 let scores = g.run();
        //                 {
        //                     pb.lock().unwrap().inc();
        //                 }
        //                 scores[0]
        //             }).collect::<Vec<_>>()
        //         }).fold(0, |acc, score| score + acc );
        //         (index, score_sum) //score = iterator
        //     })
        // }).collect();
        // let mut avg_scores: Vec<(usize, f32)> = threads.into_iter().map(|handle| {
        //     match handle.join().ok() {
        //         Some((index, score_sum)) => {
        //             let avg_score = score_sum as f32 / (games_per_ref * self.score_references.len()) as f32;
        //             (index, avg_score)
        //         }, None => {
        //             println!("ERROR - Couldn't join thread handle!");
        //             (self.current_generation.len()+1, -100.0) //Use a non-existing index! TODO: This may cause problems!!
        //         }
        //     }
        // }).collect();

        let pb = Arc::new(Mutex::new(ProgressBar::new(self.current_generation.len()*self.score_references.len()*games_per_ref)));

        let generation_size = self.current_generation.len();
        let current_generation = self.current_generation.clone();
        self.current_generation = Vec::with_capacity(generation_size);

        let score_references_amount = self.score_references.len();
        let score_references = self.score_references.clone();//.into_iter().map(|score_ref| {
        //     Arc::new(score_ref)
        // }).collect::<Vec<_>>();
        self.score_references = Vec::with_capacity(score_references_amount);

        let contestants_per_thread: usize = (generation_size as f32 / CPU_CORES as f32).ceil() as usize;
        let mut threads = Vec::with_capacity(self.current_generation.len() / contestants_per_thread);
        current_generation.into_iter().enumerate().fold( Vec::new(), |mut contestants, (index, contestant)| {
            if (index + 1) % contestants_per_thread == 0 {
                let score_references = score_references.clone(); //Create copies of the mutexes not the sfs themselves
                let pb = pb.clone();
                contestants.push(contestant);
                threads.push(thread::spawn(move || {
                    //Run those games
                    contestants.into_iter().map(move |contestant| {
                        let score_sum = score_references.iter().flat_map(|score_ref| {
                            //let reference = &score_ref_mutex.lock().unwrap(); // TODO: Dont use unwrap!

                            (0..games_per_ref).map(|game_id| {
                                //TODO: Use the same game field for the same generation.
                                let score;
                                if game_id%2 == 0 { // Change sides for every second game
                                    score = Game::new_random(&contestant, score_ref).run()[0];
                                } else {
                                    score = Game::new_random(score_ref, &contestant).run()[1];
                                }
                                pb.lock().unwrap().inc();
                                score
                            }).collect::<Vec<_>>()
                        }).fold(0, |acc, score| score + acc );
                        let avg_score = score_sum as f32 / (games_per_ref * score_references.len()) as f32;

                        (contestant, avg_score)
                    }).collect::<Vec<_>>()
                }));
                Vec::new()
            } else {
                contestants.push(contestant);
                contestants
            }
        });

        let mut results = threads.into_iter().flat_map(|thread_handle| {
            match thread_handle.join().ok() {
                Some(contestants) => {
                    contestants
                }, None => {
                    println!("ERROR - Couldn't join thread handle!");
                    Vec::new()
                    //(self.current_generation.len()+1, -100.0) //Use a non-existing index! TODO: This may cause problems!!
                }
            }
        }).collect::<Vec<_>>();

        results.sort_by(|a, b| if a.1 < b.1 {Ordering::Greater} else {Ordering::Less});

        println!("Best average score of generation is: {}", results[0].1);

        self.score_references = results.iter().take(score_references_amount).map(|score_reference| {
            score_reference.0.clone()
        }).collect();

        let mutation_per_survivor = (generation_size - self.survivor_count) / self.survivor_count;
        self.current_generation = results.into_iter().take(self.survivor_count).flat_map(|survivor| {
            let survivor = survivor.0;
            let mut mutations = (0..mutation_per_survivor).map(|_| {
                let mut mutation = survivor.clone();
                mutation.mutate(self.mutation_amount, self.mutation_strength, rng);
                mutation
            }).collect::<Vec<_>>();

            mutations.push(survivor);
            mutations
        }).collect::<Vec<_>>();

        // let mut pb = ProgressBar::new(self.current_generation.len()*self.score_references.len()*games_per_ref);
        // let mut avg_scores: Vec<(usize, f32)> = self.current_generation.iter().enumerate().map(|(index, contestant)| {
        //
        //     let score_sum =
        //         self.score_references.iter().flat_map(|reference| { // TODO: Run this in multiple threads
        //             (0..games_per_ref).map(|game_id| {
        //                 //TODO: Use the same game field for the same generation.
        //                 let score;
        //                 if game_id%2 == 0 { // Change sides for every second game
        //                     g = Game::new_random(contestant, reference).run()[0];
        //                 } else {
        //                     g = Game::new_random(reference, contestant).run()[1];
        //                 }
        //                 pb.inc();
        //                 score
        //             }).collect::<Vec<_>>()
        //         }).fold(0, |acc, score| score + acc );
        //
        //     let avg_score = score_sum as f32 / (games_per_ref * self.score_references.len()) as f32;
        //     (index, avg_score)
        //
        // }).collect();
        //
        // avg_scores.sort_by(|a, b| if a.1 < b.1 {Ordering::Greater} else {Ordering::Less});
        //
        // let new_score_refs = avg_scores
        //     .iter().take(self.score_references.len())
        //     .map(|&(index, _)| self.current_generation[index].clone())
        //     .collect();
        //
        // let generation_size = self.current_generation.len();
        // let mutation_per_survivor = (generation_size - self.survivor_count) / self.survivor_count;
        //
        // let new_generation = avg_scores.iter().take(self.survivor_count).flat_map(|&(index, _)| {
        //     let survivor = self.current_generation[index].clone();
        //     let mut mutations = (0..mutation_per_survivor).map(|_| {
        //         let mut mutation = survivor.clone();
        //         mutation.mutate(self.mutation_amount, self.mutation_strength, rng);
        //         mutation
        //     }).collect::<Vec<_>>();
        //
        //     mutations.push(survivor);
        //     mutations
        // }).collect::<Vec<_>>();
        //
        // if !(generation_size == new_generation.len()) {
        //     println!("GENERATION SIZE CHANGED {} ---> {}", self.current_generation.len(), new_generation.len());
        // }
        //
        // println!("Best average score of generation is: {}", avg_scores[0].1);
        //
        // let best_network = self.current_generation[avg_scores[0].0].clone();
        //
        // self.current_generation = new_generation;
        // self.score_references = new_score_refs;
        // best_network
    }
}
