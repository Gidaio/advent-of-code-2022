use std::fs;
use std::io;
use std::io::BufRead;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

struct Game {
    opponent_choice: Choice,
    player_choice: Choice,
}

pub fn calculate_score_of_strategy_guide() -> io::Result<usize> {
    let games = parse_input("inputs/day2.txt")?;
    let total_score = calculate_score_of_all_games(&games);

    Ok(total_score)
}

fn parse_input(path: &str) -> io::Result<Vec<Game>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut games: Vec<Game> = vec![];
    for line_result in reader.lines() {
        let line = line_result?;
        // println!("Line: {}", line);
        let parts: Vec<&str> = line.split(' ').collect();
        // println!("Parts: {:?}", parts);

        let opponent_choice = match parts[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Unrecognized opponent choice!"),
        };

        let player_choice = match parts[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!("Unrecognized player choice!"),
        };

        games.push(Game {
            opponent_choice,
            player_choice,
        });
    }

    Ok(games)
}

fn calculate_score_of_all_games(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| calculate_score_of_one_game(game))
        .sum()
}

fn calculate_score_of_one_game(game: &Game) -> usize {
    let choice_score = match game.player_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let win_score = match game.opponent_choice {
        Choice::Rock => match game.player_choice {
            Choice::Rock => 3,
            Choice::Paper => 6,
            Choice::Scissors => 0,
        },
        Choice::Paper => match game.player_choice {
            Choice::Rock => 0,
            Choice::Paper => 3,
            Choice::Scissors => 6,
        },
        Choice::Scissors => match game.player_choice {
            Choice::Rock => 6,
            Choice::Paper => 0,
            Choice::Scissors => 3,
        },
    };

    choice_score + win_score
}
