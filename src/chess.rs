use std::str::FromStr;

use indoc::indoc;
use lazy_static::lazy_static;
use regex::Regex;

use crate::elo::Rating;

// a chess player - the minimum data we care about, just name and rating
#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub rating: Rating,
}

// a single chess game, white/black players and the outcome (actual or simulated)
#[derive(Clone, Debug)]
pub struct Game {
    pub white: Player,
    pub black: Player,
    pub outcome: Outcome,
}

// possible outocmes of a chess game
#[derive(Clone, Debug)]
pub enum Outcome {
    WhiteWin,
    BlackWin,
    Draw,
}

// this regex captures all the possible fields in a PGN as exported from chess.com's API
lazy_static! {
    static ref RE: Regex = Regex::new(indoc! {r#"
        \[Event\s+"(?P<Event>.+?)"\]\s*
        \[Site\s+"(?P<Site>.+?)"\]\s*
        \[Date\s+"(?P<Date>.+?)"\]\s*
        \[Round\s+"(?P<Round>.+?)"\]\s*
        \[White\s+"(?P<White>.+?)"\]\s*
        \[Black\s+"(?P<Black>.+?)"\]\s*
        \[Result\s+"(?P<Result>.+?)"\]\s*
        \[CurrentPosition\s+"(?P<CurrentPosition>.+?)"\]\s*
        \[Timezone\s+"(?P<Timezone>.+?)"\]\s*
        \[ECO\s+"(?P<ECO>.+?)"\]\s*
        \[ECOUrl\s+"(?P<ECOUrl>.+?)"\]\s*
        \[UTCDate\s+"(?P<UTCDate>.+?)"\]\s*
        \[UTCTime\s+"(?P<UTCTime>.+?)"\]\s*
        \[WhiteElo\s+"(?P<WhiteElo>.+?)"\]\s*
        \[BlackElo\s+"(?P<BlackElo>.+?)"\]\s*
        \[TimeControl\s+"(?P<TimeControl>.+?)"\]\s*
        \[Termination\s+"(?P<Termination>.+?)"\]\s*
        \[StartTime\s+"(?P<StartTime>.+?)"\]\s*
        \[EndDate\s+"(?P<EndDate>.+?)"\]\s*
        \[EndTime\s+"(?P<EndTime>.+?)"\]\s*
        \[Link\s+"(?P<Link>.+?)"\]\s*
        (?P<PGN>.+?)\n
    "#})
    .unwrap();
}

// given a sting (expected to be a dump of chess.com's API), parse and return a list of games
pub fn parse_games(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for caps in RE.captures_iter(input) {
        let white_player = Player {
            name: caps["White"].to_string(),
            rating: Rating::from_str(caps["WhiteElo"].to_string().as_str()).unwrap(),
        };
        let black_player = Player {
            name: caps["Black"].to_string(),
            // rating: caps["BlackElo"].to_string().as_str().into(),
            rating: Rating::from_str(caps["BlackElo"].to_string().as_str()).unwrap(),
        };
        let outcome = match &caps["Result"] {
            "1-0" => Outcome::WhiteWin,
            "0-1" => Outcome::BlackWin,
            "1/2-1/2" => Outcome::Draw,
            _ => continue, // or handle unknown result
        };

        games.push(Game {
            white: white_player,
            black: black_player,
            outcome,
        });
    }

    games
}
