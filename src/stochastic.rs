use rand::Rng;

use crate::{
    chess::{Game, Outcome},
    elo::{calculate_expected_outcome, Rating},
};

// simulates a single game
fn simulate_game(white_elo: &Rating, black_elo: &Rating) -> Outcome {
    // Get expected outcomes
    let e = calculate_expected_outcome(white_elo, black_elo);

    // Roll the dice
    let mut rng = rand::thread_rng();
    let random_value = rng.gen::<f64>();

    // Determine and return outcome
    if random_value <= e.white {
        Outcome::WhiteWin
    } else if random_value <= (e.white + e.black) {
        Outcome::BlackWin
    } else {
        Outcome::Draw
    }
}

// simulate a series of games and return a new simulated `Vec<Game>` series
pub fn simulate_games(actual: &[Game]) -> Vec<Game> {
    actual
        .iter()
        .map(|g| Game {
            white: g.white.to_owned(),
            black: g.black.to_owned(),
            outcome: simulate_game(&g.white.rating, &g.black.rating),
        })
        .collect()
}
