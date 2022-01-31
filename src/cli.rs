use clap::{App, Arg};

pub fn create_wordlebot_cli_app() -> App<'static> {
    App::new("wordlebot")
        .version("0.1")
        .author("Bryan G. <bryan@bryan.sh>")
        .about(
            "Simple wordle solving bot. This bot can be used in three different
modes of operation.

 - SOLVE:
    Solve for a single word

 - SIMULATE:
    Simulates solving the entire dictionary and reports the average
    number of attempts to solve.

 - OPTIMAL:
    Simulates using every possible word to find the optimal starting
    guess along with its average.

http://github.com/gilbertw1/wordlebot",
        )
        .arg(
            Arg::new("dictionary")
                .short('d')
                .long("dictionary")
                .takes_value(true)
                .value_name("DICTIONARY")
                .help(
                    "Dictionary to use. (If not specified looks for dictionary.txt in current dir)",
                ),
        )
        .arg(
            Arg::new("emoji")
                .short('e')
                .long("emoji")
                .help("If present, print emoji results instead of raw characters"),
        )
        .arg(
            Arg::new("guess")
                .short('g')
                .long("guess")
                .takes_value(true)
                .value_name("GUESS")
                .help("Initial guess to use when solving solutions (If running SOLVE or SIMULATE)"),
        )
        .arg(
            Arg::new("hard-mode")
                .long("hard")
                .help("If present, run in hard mode"),
        )
        .arg(
            Arg::new("operation")
                .value_name("OPERATION")
                .help("Operation to run")
                .index(1)
                .possible_values(&["solve", "simulate", "optimal"])
                .required(true),
        )
        .arg(
            Arg::new("solution")
                .value_name("SOLUTION")
                .help("Solution to solve for. (If running SOLVE)")
                .index(2),
        )
}
