use std::str::FromStr;

fn main() {
    let instructions = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &[Instruction]) {
    let interesting_signal_strengths = vec![20, 60, 100, 140, 180, 220];
    let mut cycle = 0;
    let mut x_register = 1;
    let mut signal_strength = Vec::new();

    for i in instructions {
        use Instruction::*;
        match i {
            Addx(x) => {
                cycle += 1;
                if interesting_signal_strengths.contains(&cycle) {
                    let strength = cycle * x_register;
                    signal_strength.push(strength);
                }

                cycle += 1;
                if interesting_signal_strengths.contains(&cycle) {
                    let strength = cycle * x_register;
                    signal_strength.push(strength);
                }
                x_register += x;
            }
            NoOp => {
                cycle += 1;
                if interesting_signal_strengths.contains(&cycle) {
                    let strength = cycle * x_register;
                    signal_strength.push(strength);
                }
            }
        };
    }

    let sum: isize = signal_strength.iter().sum();
    println!("Part 1: {}", sum);
}

fn part2(instructions: &[Instruction]) {
    let sprite_size = 3;
    let cols = 40;
    let rows = 6;
    let mut cycle: usize = 0;
    let mut x_register = 1;

    let mut screen = vec!['.'; cols * rows];

    let mut sprite: Vec<char> = vec![vec!['#'; sprite_size], vec!['.'; cols - sprite_size]]
        .into_iter()
        .flatten()
        .collect();
    dbg!(sprite.len());

    for i in instructions {
        use Instruction::*;
        match i {
            Addx(x) => {
                screen[cycle] = sprite[cycle % 40];
                cycle += 1;

                screen[cycle] = sprite[cycle % 40];
                cycle += 1;
                x_register += x;
                for (i, s) in sprite.iter_mut().enumerate() {
                    if i == x_register as usize
                        || i == (x_register + 1) as usize
                        || i == (x_register - 1) as usize
                    {
                        *s = '#';
                    } else {
                        *s = '.';
                    }
                }
            }
            NoOp => {
                screen[cycle] = sprite[cycle % 40];
                cycle += 1;
            }
        };
    }

    for row in 0..rows {
        let start = row * cols;
        let end = start + cols;
        let row = &screen[start..end];
        let row = row.iter().map(|c| c.to_string()).collect::<String>();
        println!("{row}");
    }
}

#[derive(Debug)]
enum Instruction {
    Addx(isize),
    NoOp,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let instruction = parts.next().unwrap();
        match instruction {
            "noop" => Ok(Instruction::NoOp),
            "addx" => {
                let value = parts.next().unwrap().parse::<isize>().unwrap();
                Ok(Instruction::Addx(value))
            }
            _ => Err(()),
        }
    }
}
