use crate::cli;

#[derive(Debug)]
pub struct WordleConfig {
    pub dictionary: Option<String>,
    pub emoji: bool,
    pub guess: Option<String>,
    pub hard_mode: bool,
    pub operation: WordleOp,
    pub solution: Option<String>,
}

impl WordleConfig {
    pub fn load() -> WordleConfig {
        let cli_app = cli::create_wordlebot_cli_app();
        let matches = cli_app.get_matches();
        WordleConfig {
            dictionary: matches.value_of("dictionary").map(|m| m.to_string()),
            emoji: matches.is_present("emoji"),
            guess: matches.value_of("guess").map(|m| m.to_string()),
            hard_mode: matches.is_present("hard-mode"),
            operation: WordleOp::from_string(
                matches
                    .value_of("operation")
                    .expect("OPERATION parameter is required!"),
            ),
            solution: matches.value_of("solution").map(|m| m.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum WordleOp {
    Solve,
    Simulate,
    Optimal,
}

impl WordleOp {
    fn from_string(value: &str) -> WordleOp {
        match value.to_lowercase().as_ref() {
            "solve" => WordleOp::Solve,
            "simulate" => WordleOp::Simulate,
            "optimal" => WordleOp::Optimal,
            _ => panic!("Invalid operation specified! Op: {}", value),
        }
    }
}
