// reader.rs
// This module provides utility functions for loading board game data from CSV files.

use crate::models::BoardGame;
use csv::ReaderBuilder;
use std::error::Error;

// Loads board game data from a CSV file and returns a Vec of BoardGame objects.
// Skips rows that fail to parse.
// Inputs
// - `path`: File path to the CSV file
// Outputs
// - Ok(Vec<BoardGame>) if successful, Err if file loading or deserialization fails
pub fn load_boardgames(path: &str) -> Result<Vec<BoardGame>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;

    // Filter out rows that failed to deserialize.
    let games = rdr.deserialize().filter_map(Result::ok) .collect();

    Ok(games)
}
