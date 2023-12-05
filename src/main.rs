// project modules
mod chess;
mod elo;
mod stochastic;

// standard imports
use std::{collections::HashMap, fs, sync::Arc};

// crate imports
use rayon::prelude::*;

// project imports
use chess::{parse_games, Game, Outcome};
use stochastic::simulate_games;

// load games into `Vec<Game>` from the chess.com API output
fn load_games() -> Vec<Game> {
    let filename = "games.pgn";

    // Read the contents of the file into a string
    let contents = fs::read_to_string(filename).unwrap();

    // Parse the contents into a Vec<Game>
    parse_games(&contents)
}

// find all the win streaks in a `Vec<Game>` for players matching `player_name`
fn generate_win_streak_histogram(player_name: &str, games: &[Game]) -> HashMap<usize, usize> {
    let mut histogram = HashMap::new();
    let mut current_streak = 0;

    for game in games {
        let is_player_white = game.white.name == player_name;
        let is_player_black = game.black.name == player_name;

        assert!(is_player_white || is_player_black);
        if !(is_player_white || is_player_black) {
            continue;
        }

        let won_as_white = is_player_white && matches!(game.outcome, Outcome::WhiteWin);
        let won_as_black = is_player_black && matches!(game.outcome, Outcome::BlackWin);

        if won_as_white || won_as_black {
            // our player won
            current_streak += 1;
        } else if current_streak > 0 {
            // our playe lost or drew, record streak and reset it
            *histogram.entry(current_streak).or_insert(0) += 1;
            current_streak = 0;
        }
    }

    // Handle the last streak if the games list ends on a winning streak
    if current_streak > 0 {
        *histogram.entry(current_streak).or_insert(0) += 1;
    }

    histogram
}

// dump the histogram to stdout
fn print_histogram(histogram: &HashMap<usize, usize>, divisor: Option<u32>) {
    let max_streak = histogram.keys().max().cloned().unwrap_or(0);

    println!("Win Streak Histogram:");
    for streak_length in 1..=max_streak {
        let count = histogram.get(&streak_length).cloned().unwrap_or(0);

        match divisor {
            Some(div) => {
                let expected_count = (count as f64) / (div as f64);
                let probability = expected_count.min(1.0) * 100.0;
                println!(
                    "Streak Length: {:>3}, Expected Count: {:>5.0}, Probability: {:>6.2}%",
                    streak_length, expected_count, probability
                );
            }
            None => {
                println!("Streak Length: {:>3}, Count: {}", streak_length, count);
            }
        }
    }
}

fn main() {
    // load actual games
    println!("Loading...");
    let games = Arc::new(load_games()); // Arc because of rayon use below
    println!("Loaded {} games", games.len());
    println!();

    println!("Actual Results Histogram:");
    let histogram = generate_win_streak_histogram("Hikaru", &games);
    print_histogram(&histogram, None);
    println!();

    // Run the simulations in parallel and collect the histograms
    println!("Simulating...");
    println!();
    let iterations = 10_000;
    let histograms: Vec<HashMap<usize, usize>> = (0..iterations)
        .into_par_iter()
        .map(|_| {
            let simulated_games = simulate_games(&games);
            generate_win_streak_histogram("Hikaru", &simulated_games)
        })
        .collect();

    // Reduce the histograms into a single histogram
    let merged_histogram = histograms
        .into_iter()
        .reduce(|mut acc, h| {
            for (k, v) in h {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        })
        .unwrap();

    // And print it!
    println!("Simulated Results ({} games):", iterations);
    print_histogram(&merged_histogram, Some(iterations));
}
