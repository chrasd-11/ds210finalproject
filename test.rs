use your_crate_name::models::BoardGame;
use your_crate_name::analyzer::analyze_average_rating;

#[test]
fn test_average_rating() {
    let games = vec![
        BoardGame {
            Name: "Test Game 1".to_string(),
            YearPublished: Some(2020),
            MinPlayers: 2,
            MaxPlayers: 4,
            MfgPlaytime: 60,
            AvgRating: Some(7.0),
        },
        BoardGame {
            Name: "Test Game 2".to_string(),
            YearPublished: Some(2021),
            MinPlayers: 3,
            MaxPlayers: 5,
            MfgPlaytime: 90,
            AvgRating: Some(8.0),
        },
    ];

    let avg = analyze_average_rating(&games).unwrap();
    assert!((avg - 7.5).abs() < f32::EPSILON);
}
