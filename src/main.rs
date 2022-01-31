extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod cli;
mod conf;
mod dict;
mod eval;
mod sim;

use conf::{WordleConfig, WordleOp};
use dict::{Dictionary, Word};
use eval::EvaluationResult;
use sim::Simulator;

fn main() {
    let config = WordleConfig::load();
    let now = Instant::now();
    let words = load_dictionary_words(&config);
    let dictionary = Dictionary::create(&words);
    run_operation(config, dictionary, words);
    println!("Elapsed: {} ms", now.elapsed().as_millis());
}

fn load_dictionary_words(config: &WordleConfig) -> Vec<Word> {
    let filename = config
        .dictionary
        .to_owned()
        .unwrap_or("dictionary.txt".to_string());
    let file = File::open(filename).expect("Error opening dictionary file!");
    BufReader::new(&file)
        .lines()
        .map(|l| Word::create(&l.unwrap()))
        .collect()
}

fn run_operation(config: WordleConfig, dictionary: Dictionary, words: Vec<Word>) {
    match config.operation {
        WordleOp::Solve => solve(config, dictionary),
        WordleOp::Simulate => simulate(config, dictionary, words),
        WordleOp::Optimal => optimal(config, dictionary, words),
    }
}

fn solve(config: WordleConfig, dictionary: Dictionary) {
    let solution = Word::create(
        &config
            .solution
            .as_ref()
            .expect("Error: SOLUTION required to run SOLVE operation."),
    );
    let simulation = Simulator::create(&dictionary, solution);
    let results = simulation.simulate(&config, None);
    print_results(&results, config.emoji);
}

fn simulate(config: WordleConfig, dictionary: Dictionary, words: Vec<Word>) {
    let mut total = 0.0;
    let now = Instant::now();
    words.iter().for_each(|word| {
        let simulation = Simulator::create(&dictionary, *word);
        let results = simulation.simulate(&config, None);
        total += results.len() as f64;
    });
    let initial = config
        .guess
        .as_ref()
        .map(|g| Word::create(g))
        .unwrap_or(dictionary.find_best_guess());
    println!(
        "Initial Guess: {}",
        initial.chars.iter().collect::<String>()
    );
    println!("Simulations Run: {}", words.len());
    println!("Average Guesses: {:.3}", total / words.len() as f64);
    println!(
        "Average Simulation Duration: {:.3} ms",
        now.elapsed().as_millis() as f64 / words.len() as f64
    );
}

fn optimal(config: WordleConfig, dictionary: Dictionary, words: Vec<Word>) {
    let mut best_score = 100.0;
    let mut best = None;
    let mut iteration_count = 0;
    words.iter().for_each(|guess| {
        let mut total = 0.0;
        words.iter().for_each(|word| {
            let simulation = Simulator::create(&dictionary, *word);
            let results = simulation.simulate(&config, Some(guess.clone()));
            total += results.len() as f64;
        });
        iteration_count += 1;
        let avg = total / words.len() as f64;
        if avg < best_score {
            best_score = avg;
            best = Some(guess);
        }

        println!("Iteration: {}", iteration_count);
        println!("Guess: {}", guess.chars.iter().collect::<String>());
        println!("Average Guesses: {:.3}", avg);
        println!("-------------------------")
    });
    println!("**************************");
    println!("******* COMPLETED ********");
    println!("**************************");
    println!(
        "Best Guess: {}",
        best.unwrap().chars.iter().collect::<String>()
    );
    println!("Average Guesses: {:.3}", best_score);
}

fn print_results(results: &Vec<EvaluationResult>, emoji: bool) {
    results.iter().for_each(|r| {
        r.sorted_results().iter().for_each(|cr| {
            if emoji {
                print!("{}", cr.format(emoji));
            } else {
                print!("{} ", cr.format(emoji));
            }
        });
        println!("");
    });
}
