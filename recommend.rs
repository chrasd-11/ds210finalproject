// recommend.rs
// This module includes functions for recommending board games based on player count and rating.

use crate::models::BoardGame;

// Recommends board games based on a desired number of players.
// Inputs:
// - `games`: A slice of `BoardGame` objects.
// - `desired_players`: The number of players for which the games should be recommended.
// Outputs:
// - Returns a `Vec<&BoardGame>` containing all board games that can support the desired player count.
// Logic:
// The function filters through the list of games, checking if the `MinPlayers` and `MaxPlayers` fields cover the requested number of players, and returns a vector of games that satisfy this condition.
pub fn recommend_games_by_players(games: &[BoardGame], desired_players: u8) -> Vec<&BoardGame> {
    games
        .iter()
        .filter(|game| {
            if let (Some(min), Some(max)) = (game.MinPlayers, game.MaxPlayers) {
                min <= desired_players && desired_players <= max
            } else {
                false
            }
        })
        .collect()
}

// Recommends board games that meet or exceed a given average rating.
// Inputs:
// - `games`: A slice of `BoardGame` objects.
// - `min_rating`: The minimum average rating to qualify.
// Outputs:
// - A `Vec<&BoardGame>` of games whose average rating is >= `min_rating`.
// Logic:
// Filters the games by rating using `unwrap_or(0.0)` to safely handle `None` values.
pub fn recommend_games_by_rating(games: &[BoardGame], min_rating: f32) -> Vec<&BoardGame> {
    games
        .iter()
        .filter(|game| game.AvgRating.unwrap_or(0.0) >= min_rating)
        .collect()
}
