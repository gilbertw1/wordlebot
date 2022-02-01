use crate::dict::Word;

#[derive(Clone, Copy, Debug)]
pub struct Evaluator {
    solution: Word,
}

impl Evaluator {
    pub fn create(solution: Word) -> Evaluator {
        Evaluator { solution }
    }

    pub fn evaluate_guess(&self, guess: &Word) -> EvaluationResult {
        let mut results = Vec::with_capacity(5);
        let mut seen = Vec::with_capacity(5);
        guess.chars.iter().enumerate().for_each(|(idx, c)| {
            if &self.solution.chars[idx] == c {
                results.push(CharResult {
                    result_type: CharResultType::Correct,
                    c: c.clone(),
                    pos: idx,
                });
                seen.push(c);
            }
        });

        guess.chars.iter().enumerate().for_each(|(idx, c)| {
            if &self.solution.chars[idx] != c {
                let count = self.solution.chars.iter().filter(|ch| c == *ch).count();
                let seen_count = seen.iter().filter(|ch| &c == *ch).count();
                if count > seen_count {
                    results.push(CharResult {
                        result_type: CharResultType::WrongSpot,
                        c: c.clone(),
                        pos: idx,
                    });
                    seen.push(c);
                } else {
                    results.push(CharResult {
                        result_type: CharResultType::NotFound,
                        c: c.clone(),
                        pos: idx,
                    });
                    seen.push(c);
                }
            }
        });

        EvaluationResult {
            char_results: [results[0], results[1], results[2], results[3], results[4]],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EvaluationResult {
    char_results: [CharResult; 5],
}

impl EvaluationResult {
    pub fn is_correct(&self) -> bool {
        self.char_results
            .iter()
            .all(|r| matches!(r.result_type, CharResultType::Correct))
    }

    pub fn matches_word(&self, word: &Word) -> bool {
        let mut eliminated = [false; 5];
        self.char_results
            .iter()
            .all(|r| r.matches_word(word, &mut eliminated))
    }

    pub fn not_contains_matches(&self, word: &Word) -> bool {
        !self.char_results.iter().any(|r| word.chars.contains(&r.c))
    }

    pub fn sorted_results(&self) -> Vec<CharResult> {
        let mut char_results = self.char_results.clone().to_vec();
        char_results.sort_by(|a, b| a.pos.cmp(&b.pos));
        char_results
    }

    pub fn correct_count(&self) -> u32 {
    self.char_results.iter().filter(|r| r.is_match()).count() as u32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CharResult {
    result_type: CharResultType,
    c: char,
    pos: usize,
}

impl CharResult {
    fn matches_word(&self, word: &Word, eliminated: &mut [bool; 5]) -> bool {
        match self.result_type {
            CharResultType::NotFound => !word
                .chars
                .iter()
                .enumerate()
                .any(|(idx, ch)| !eliminated[idx] && ch == &self.c),
            CharResultType::Correct => {
                let matches = word.chars[self.pos] == self.c;
                if matches {
                    eliminated[self.pos] = true;
                }
                matches
            }
            CharResultType::WrongSpot => {
                let mut idx = 0;
                let mut found = false;
                while idx < 5 {
                    if idx == self.pos && word.chars[idx] == self.c {
                        return false;
                    }
                    if !eliminated[idx] && word.chars[idx] == self.c {
                        found = true;
                        eliminated[idx] = true;
                        break;
                    }
                    idx += 1;
                }
                found
            }
        }
    }

    pub fn format(&self, emoji: bool) -> String {
        self.result_type.format(self.c, emoji)
    }

    fn is_match(&self) -> bool {
        match self.result_type {
            CharResultType::NotFound => false,
            CharResultType::Correct => true,
            CharResultType::WrongSpot => true,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum CharResultType {
    NotFound,
    Correct,
    WrongSpot,
}

impl CharResultType {
    fn format(&self, c: char, emoji: bool) -> String {
        match self {
            CharResultType::NotFound => {
                if emoji {
                    "⬛".to_string()
                } else {
                    format!("-{}", c)
                }
            }
            CharResultType::Correct => {
                if emoji {
                    "🟩".to_string()
                } else {
                    format!("+{}", c)
                }
            }
            CharResultType::WrongSpot => {
                if emoji {
                    "🟨".to_string()
                } else {
                    format!("~{}", c)
                }
            }
        }
    }
}
