#[derive(Debug)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameEnd {
    Win,
    Lose,
    Draw,
}

fn main() {
    let total_score = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            let mut line = line.split(' ');
            let opponent_move = line.next().unwrap();
            let opponent_move = parse_abc(opponent_move);

            let game_end = line.next().unwrap();
            let game_end = parse_xyz_to_game_end(game_end);

            let my_move = calculate_my_move(&opponent_move, &game_end);

            let my_score = calculate_win_score(&my_move, &opponent_move);

            let score = match my_move {
                GameMove::Rock => my_score + 1,
                GameMove::Paper => my_score + 2,
                GameMove::Scissors => my_score + 3,
            };

            acc + score
        });

    dbg!(total_score);
}

fn parse_abc(s: &str) -> GameMove {
    use GameMove::*;
    if s == "A" {
        Rock
    } else if s == "B" {
        Paper
    } else if s == "C" {
        Scissors
    } else {
        panic!("Invalid move")
    }
}

// used for part 1
fn _parse_xyz(s: &str) -> GameMove {
    use GameMove::*;
    if s == "X" {
        Rock
    } else if s == "Y" {
        Paper
    } else if s == "Z" {
        Scissors
    } else {
        panic!("Invalid move")
    }
}

fn parse_xyz_to_game_end(s: &str) -> GameEnd {
    use GameEnd::*;
    if s == "X" {
        Lose
    } else if s == "Y" {
        Draw
    } else if s == "Z" {
        Win
    } else {
        panic!("Invalid game end: {}", s);
    }
}

fn calculate_win_score(my_move: &GameMove, opponent_move: &GameMove) -> i32 {
    use GameMove::*;
    match (my_move, opponent_move) {
        (Rock, Scissors) => 6,
        (Rock, Paper) => 0,
        (Rock, Rock) => 3,
        (Paper, Rock) => 6,
        (Paper, Scissors) => 0,
        (Paper, Paper) => 3,
        (Scissors, Paper) => 6,
        (Scissors, Rock) => 0,
        (Scissors, Scissors) => 3,
    }
}

fn calculate_my_move(opponent_move: &GameMove, game_end: &GameEnd) -> GameMove {
    use GameEnd::*;
    use GameMove::*;
    match (opponent_move, game_end) {
        (Rock, Win) => Paper,
        (Rock, Draw) => Rock,
        (Rock, Lose) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Draw) => Paper,
        (Paper, Lose) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Draw) => Scissors,
        (Scissors, Lose) => Paper,
    }
}
