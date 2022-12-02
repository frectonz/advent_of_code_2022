fn main() {
    let total_score = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let mut line = line.split(" ");
            let opponent_move = line.next().unwrap();
            let opponent_move = parse_my_move(opponent_move);

            let game_end = line.next().unwrap();
            let game_end = parse_game_end(game_end);

            let my_move = calculate_my_move(opponent_move, game_end);

            let my_score = calculate_win_score(my_move, opponent_move);

            let score = match my_move {
                "rock" => my_score + 1,
                "paper" => my_score + 2,
                "scissors" => my_score + 3,
                _ => my_score,
            };

            acc + score
        });

    dbg!(total_score);
}

fn parse_my_move(s: &str) -> &str {
    if s == "A" {
        "rock"
    } else if s == "B" {
        "paper"
    } else if s == "C" {
        "scissors"
    } else {
        panic!("Invalid move: {}", s);
    }
}

fn parse_opponent_move(s: &str) -> &str {
    if s == "X" {
        "rock"
    } else if s == "Y" {
        "paper"
    } else if s == "Z" {
        "scissors"
    } else {
        panic!("Invalid move: {}", s);
    }
}

fn parse_game_end(s: &str) -> &str {
    if s == "X" {
        "LOSE"
    } else if s == "Y" {
        "DRAW"
    } else if s == "Z" {
        "WIN"
    } else {
        panic!("Invalid game end: {}", s);
    }
}

fn calculate_win_score(my_move: &str, opponent_move: &str) -> i32 {
    match (my_move, opponent_move) {
        ("rock", "scissors") => 6,
        ("rock", "paper") => 0,
        ("rock", "rock") => 3,
        ("paper", "rock") => 6,
        ("paper", "scissors") => 0,
        ("paper", "paper") => 3,
        ("scissors", "paper") => 6,
        ("scissors", "rock") => 0,
        ("scissors", "scissors") => 3,
        _ => panic!("Invalid move combination: {} {}", my_move, opponent_move),
    }
}

fn calculate_my_move<'a>(opponent_move: &'a str, game_end: &'a str) -> &'a str {
    match (opponent_move, game_end) {
        ("rock", "WIN") => "paper",
        ("rock", "DRAW") => "rock",
        ("rock", "LOSE") => "scissors",
        ("paper", "WIN") => "scissors",
        ("paper", "DRAW") => "paper",
        ("paper", "LOSE") => "rock",
        ("scissors", "WIN") => "rock",
        ("scissors", "DRAW") => "scissors",
        ("scissors", "LOSE") => "paper",
        _ => panic!("Invalid move combination: {} {}", opponent_move, game_end),
    }
}
