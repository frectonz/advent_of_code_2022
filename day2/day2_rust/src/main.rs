fn main() {
    let total_score = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let mut line = line.split(" ");
            let opponent_move = line.next().unwrap();
            let opponent_move = parse_my_move(opponent_move);

            let my_move = line.next().unwrap();
            let my_move = parse_opponent_move(my_move);

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
