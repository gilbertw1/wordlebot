use std::collections::HashMap;

use crate::eval::EvaluationResult;

static LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

lazy_static! {
    static ref LETTERS_MAP: HashMap<char, [u32; 5]> =
        LETTERS.iter().map(|l| (l.clone(), [0; 5])).collect();
}

#[derive(Clone, Debug)]
pub struct Dictionary {
    words: Vec<DictionaryWord>,
    stats: LetterStats,
}

impl Dictionary {
    pub fn create(words: &Vec<Word>) -> Dictionary {
        let stats = LetterStats::create(&words);
        let dict_words = words
            .iter()
            .map(|word| DictionaryWord {
                word: word.clone(),
                score: stats.calculate_score(word),
            })
            .collect();
        Dictionary {
            words: dict_words,
            stats,
        }
    }

    pub fn find_best_guess(&self) -> Word {
        self.words
            .iter()
            .max_by(|x, y| x.score.cmp(&y.score))
            .map(|dw| dw.word)
            .unwrap()
    }

    pub fn filter_based_on_results(&self, results: &Vec<EvaluationResult>) -> Dictionary {
        Dictionary::create(
            &self
                .words
                .iter()
                .filter(|w| results.iter().all(|r| r.matches_word(&w.word)))
                .map(|w| w.word.clone())
                .collect(),
        )
    }

    pub fn find_best_guess_not_matching(&self, results: &Vec<EvaluationResult>) -> Word {
        self.words
            .iter()
            .filter(|w| Dictionary::word_not_contains_matches(&w.word, results))
            .max_by(|x, y| x.score.cmp(&y.score))
            .map(|dw| dw.word.clone())
            .to_owned()
            .unwrap()
    }

    pub fn word_not_contains_matches(word: &Word, results: &Vec<EvaluationResult>) -> bool {
        results.iter().all(|r| r.not_contains_matches(word))
    }
}

#[derive(Clone, Copy, Debug)]
struct DictionaryWord {
    word: Word,
    score: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Word {
    pub chars: [char; 5],
}

impl Word {
    pub fn create(word: &str) -> Word {
        let mut chars = word.chars();
        Word {
            chars: [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ],
        }
    }
}

#[derive(Clone, Debug)]
struct LetterStats {
    stats: HashMap<char, [u32; 5]>,
}

impl LetterStats {
    pub fn create(words: &Vec<Word>) -> LetterStats {
        let mut stats = LETTERS_MAP.clone();
        words.iter().for_each(|word| {
            word.chars.iter().enumerate().for_each(|(idx, c)| {
                let stat = stats.get_mut(c).unwrap();
                stat[idx] = stat[idx] + 1;
            });
        });
        LetterStats { stats }
    }

    pub fn calculate_score(&self, word: &Word) -> u32 {
        let mut score = 0;
        let mut seen = 1;
        word.chars.iter().enumerate().for_each(|(idx, c)| {
            if seen % *c as u32 == 0 {
                score += self.stats.get(c).unwrap()[idx] / 2;
            } else {
                score += self.stats.get(c).unwrap()[idx];
                seen *= *c as u32;
            }
        });
        score
    }
}
