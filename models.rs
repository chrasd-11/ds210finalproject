// models.rs
// Defines data structures used in the project, particularly the `BoardGame` struct used for CSV deserialization.
use serde::Deserialize;

// Struct representing a board game as defined in the dataset.
// Fields correspond to CSV headers, using exact names for compatibility with serde.
#[derive(Debug, Deserialize, Clone)]
pub struct BoardGame {
    // The name of the game
    pub Name: String,
    // Year the game was published (optional
    pub YearPublished: Option<u16>,
    // Minimum number of players
    pub MinPlayers: Option<u8>,
    // Maximum number of players
    pub MaxPlayers: Option<u8>,
    // Manufacturer's suggested playtime
    pub MfgPlaytime: u16,
    // Average user rating (optional)
    pub AvgRating: Option<f32>,
}