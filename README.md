wordlebot
=========

A Simple wordle bot that solves wordle puzzles.

Installation
------------

### Source

Clone:

    git clone git@github.com:gilbertw1/wordlebot.git

Build (Requires [Rust](https://www.rust-lang.org/)):

    cargo build --release

Run:

    ./target/release/wordlebot solve adore



Usage
-----

```
wordlebot 0.1
Bryan G. <bryan@bryan.sh>
Simple wordle solving bot. This bot can be used in three different
modes of operation.

 - SOLVE:
    Solve for a single word

 - SIMULATE:
    Simulates solving the entire dictionary and reports the average
    number of attempts to solve.

 - OPTIMAL:
    Simulates using every possible word to find the optimal starting
    guess along with its average.

http://github.com/gilbertw1/wordlebot

USAGE:
    wordlebot [OPTIONS] <OPERATION> [SOLUTION]

ARGS:
    <OPERATION>    Operation to run [possible values: solve, simulate, optimal]
    <SOLUTION>     Solution to solve for. (If running SOLVE)

OPTIONS:
    -d, --dictionary <DICTIONARY>    Dictionary to use. (If not specified looks for dictionary.txt
                                     in current dir)
    -e, --emoji                      If present, print emoji results instead of raw characters
    -g, --guess <GUESS>              Initial guess to use when solving solutions (If running SOLVE
                                     or SIMULATE)
    -h, --help                       Print help information
        --hard                       If present, run in hard mode
    -V, --version                    Print version information
```
