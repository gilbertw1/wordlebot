use crate::conf::WordleConfig;
use crate::dict::{Dictionary, Word};
use crate::eval::{EvaluationResult, Evaluator};

pub struct Simulator<'a> {
    evaluator: Evaluator,
    dictionary: &'a Dictionary,
}

impl<'a> Simulator<'a> {
    pub fn create(dictionary: &'a Dictionary, solution: Word) -> Simulator<'a> {
        Simulator {
            evaluator: Evaluator::create(solution),
            dictionary,
        }
    }

    pub fn simulate(
        &self,
        config: &WordleConfig,
        initial_guess: Option<Word>,
    ) -> Vec<EvaluationResult> {
        let guess = initial_guess.unwrap_or(
            config
                .guess
                .as_ref()
                .map(|g| Word::create(g))
                .unwrap_or(self.dictionary.find_best_guess()),
        );
        self.run_simulation(config, &self.dictionary, Vec::new(), &guess, &self.dictionary)
    }

    fn run_simulation(
        &self,
        config: &WordleConfig,
        dictionary: &Dictionary,
        mut results: Vec<EvaluationResult>,
        guess: &Word,
        og_dict: &Dictionary,
    ) -> Vec<EvaluationResult> {
        let result = self.evaluator.evaluate_guess(guess);
        results.push(result);
        if result.is_correct() {
            results
        } else {
            let filtered_dict = dictionary.filter_based_on_results(&results);
            let guess = if results.len() == 1 && !config.hard_mode {
                dictionary.find_best_guess_not_matching(&results)
            } else if results.len() == 2 && !config.hard_mode && Simulator::correct_count(&results) < 4 {
                og_dict.find_best_guess_not_matching(&results)
            } else {
                filtered_dict.find_best_guess()
            };
            self.run_simulation(config, &filtered_dict, results, &guess, og_dict)
        }
    }

    fn correct_count(results: &Vec<EvaluationResult>) -> u32 {
        results.iter().map(|r| r.correct_count()).sum()
    }
}
