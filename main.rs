mod analysis;
mod models;
mod recommend;
mod reader;

use clap::Parser;
use models::BoardGame;
use reader::load_boardgames;

#[derive(Parser)]
#[command(name = "Board Game Analyzer")]
#[command(about = "Analyze and recommend board games based on dataset", long_about = None)]
struct Cli {
    #[arg(short, long)]
    recommend: Option<u8>,

    #[arg(short, long)]
    min_rating: Option<f32>,

    #[arg(short, long, default_value = "data/boardgames.csv")]
    file: String,
}

fn main() {
    let cli = Cli::parse();

    let games: Vec<BoardGame> = load_boardgames(&cli.file)
        .expect("Failed to load board games");

    println!("Total games loaded: {}", games.len());

    if let Some(rating) = analysis::analyze_average_rating(&games) {
        println!("Average rating: {:.2}", rating);
    }

    let (highest, lowest) = analysis::find_extreme_rated_games(&games);
    if let Some(high) = highest {
        println!("Highest rated: {} ({:.2})", high.Name, high.AvgRating.unwrap_or(0.0));
    }
    if let Some(low) = lowest {
        println!("Lowest rated: {} ({:.2})", low.Name, low.AvgRating.unwrap_or(0.0));
    }

    if let Some(avg_time) = analysis::average_play_time(&games) {
        println!("Average playtime: {:.1} minutes", avg_time);
    }

    if let Some(players) = cli.recommend {
        println!("\n Recommendations for {} players:", players);
        let recommended = recommend::recommend_games_by_players(&games, players);
        for game in recommended.iter().take(5) {
            println!("{} ({}–{} players)", game.Name, game.MinPlayers.map_or("N/A".into(), |v| v.to_string()), game.MaxPlayers.map_or("N/A".into(), |v| v.to_string()));        
        }
    }

    if let Some(min_rating) = cli.min_rating {
        println!("\n Games with rating ≥ {:.1}:", min_rating);
        let high_rated = recommend::recommend_games_by_rating(&games, min_rating);
        for game in high_rated.iter().take(5) {
            println!("{} ({}–{} players)", game.Name, game.MinPlayers.map_or("N/A".into(), |v| v.to_string()), game.MaxPlayers.map_or("N/A".into(), |v| v.to_string())); 
        }
    }    
}

