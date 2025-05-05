// analysis.rs
// This module contains functions for analyzing board games, such as calculating average ratings, finding extreme rated games, and counting games based on player numbers.

use crate::models::BoardGame;
use std::collections::HashMap;

// Analyzes the average rating of a list of board games.
// Inputs:
// - `games`: A slice of `BoardGame` objects representing the list of games to analyze.
// Outputs:
// - Returns an `Option<f32>`: Some(average rating) if valid ratings are present, or None if no ratings are available.
// Logic:
// This function iterates through all board games, summing the average ratings and counting how many games have a valid rating. It then calculates the average by dividing the sum by the count of valid ratings.
pub fn analyze_average_rating(games: &[BoardGame]) -> Option<f32> {
    let (sum, count) = games.iter().fold((0.0, 0), |(sum, count), game| {
        if let Some(rating) = game.AvgRating {
            (sum + rating, count + 1)
        } else {
            (sum, count)
        }
    });

    if count > 0 {
        Some(sum / count as f32)
    } else {
        None
    }
}

// Finds the highest and lowest rated games from a list.
// Inputs:
// - `games`: A slice of `BoardGame` objects.
// Outputs:
// - A tuple containing:
//  - `Some(&BoardGame)` with the highest rated game,
//  - `Some(&BoardGame)` with the lowest rated game.
//  - If no ratings are found, both values are `None`.
// Logic:
// Iterates through the list and compares ratings, tracking the current highest and lowest.
pub fn find_extreme_rated_games(games: &[BoardGame]) -> (Option<&BoardGame>, Option<&BoardGame>) {
    let mut highest: Option<&BoardGame> = None;
    let mut lowest: Option<&BoardGame> = None;

    for game in games {
        if let Some(rating) = game.AvgRating {
            if highest.is_none() || rating > highest.unwrap().AvgRating.unwrap_or(0.0) {
                highest = Some(game);
            }
            if lowest.is_none() || rating < lowest.unwrap().AvgRating.unwrap_or(f32::MAX) {
                lowest = Some(game);
            }
        }
    }

    (highest, lowest)
}

// Counts how many games support each possible number of players.
// Inputs:
// - `games`: A slice of `BoardGame` objects.
// Outputs:
// - Returns a `HashMap<u8, usize>` where the key is the player count and the value is how many games support it.
// Logic:
// For each game with a valid min and max player range, iterate over all supported counts and increment a frequency map.
pub fn count_games_per_player_count(games: &[BoardGame]) -> HashMap<u8, usize> {
    let mut player_count_map = HashMap::new();

    for game in games {
        if let (Some(min), Some(max)) = (game.MinPlayers, game.MaxPlayers) {
            for players in min..=max {
                *player_count_map.entry(players).or_insert(0) += 1;
            }
        }
    }

    player_count_map
}

// Calculates the average manufacturer play time for all games.
// Inputs:
// - `games`: A slice of `BoardGame` objects.
// Outputs:
// - Returns `Some(f32)` if any games are present, or `None` if empty.
// Logic:
// Sums up all manufacturer play times and divides by the number of games.
pub fn average_play_time(games: &[BoardGame]) -> Option<f32> {
    let (total_time, count) = games.iter().fold((0, 0), |(total, count), game| {
        (total + game.MfgPlaytime as usize, count + 1)
    });

    if count > 0 {
        Some(total_time as f32 / count as f32)
    } else {
        None
    }
}
