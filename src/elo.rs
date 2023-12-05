// standard imports
use std::{num::ParseIntError, str::FromStr};

// crate imports
use assert_approx_eq::assert_approx_eq;

// probabilities of white or black winning or drawing
#[derive(Debug)]
pub struct ExpectedOutcome {
    pub white: f64,
    pub black: f64,
    pub draw: f64,
}

// a new type for an Elo rating
#[derive(Clone, Debug)]
pub struct Rating(pub u32);

// create a `Rating` from a string (used during loading/parsing)
impl FromStr for Rating {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<u32>()?; // Or whatever the primitive type should be
        Ok(Self(num)) // Replace with the correct method to construct a Rating
    }
}

// estimate probability of draw given two Elo ratings
fn probability_of_draw(white_elo: &Rating, black_elo: &Rating) -> f64 {
    // Constants for draw probability estimation
    let max_draw_probability = 0.18; // Upper limit of draw probability
    let rating_difference_sensitivity = 0.05; // Adjusts sensitivity to rating difference
    let rating_diff = (black_elo.0 as f64 - white_elo.0 as f64).abs();
    max_draw_probability / (1.0 + rating_difference_sensitivity * rating_diff)
}

// estimate probability of white winning, given two Elo ratings
fn probability_of_white_win(white_elo: &Rating, black_elo: &Rating) -> f64 {
    // Expected outcomes for players A and B
    // E_a = 1 / (1 + 10^((b - a) / 400)
    // E_b = 1 - E_a
    1.0 / (1.0 + 10.0f64.powf((black_elo.0 as f64 - white_elo.0 as f64) / 400.0))
}

// estimate probability of black winning given white's probability of winning
fn probability_of_black_win(probability_of_white_win: f64) -> f64 {
    1.0 - probability_of_white_win
}

// given two ratings, compute probabilities of win/loss/draw and return in `ExpectedOutcome`
pub fn calculate_expected_outcome(white_elo: &Rating, black_elo: &Rating) -> ExpectedOutcome {
    // Estimate win probabilities
    let mut p_white = probability_of_white_win(white_elo, black_elo);
    let mut p_black = probability_of_black_win(p_white);

    // Estimate draw probability based on rating difference
    let p_draw = probability_of_draw(white_elo, black_elo);

    // Adjust win probabilities to account for draw probability
    p_white *= 1.0 - p_draw;
    p_black *= 1.0 - p_draw;

    // Sanity check
    assert_approx_eq!((p_white + p_black + p_draw), 1.0);

    ExpectedOutcome {
        // Estimate win probabilities
        white: p_white,
        black: p_black,
        draw: p_draw,
    }
}
