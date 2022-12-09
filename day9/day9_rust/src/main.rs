use ::lending_iterator::prelude::*;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let motions = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let mut line = line.trim().split(" ");
            let motion = line.next()?.parse::<Motion>().ok()?;
            let count = line.next()?.parse::<usize>().ok()?;

            Some(vec![motion; count])
        })
        .flatten()
        .collect::<Vec<_>>();

    part1(motions.clone());
    part2(motions);
}

fn part1(motions: Vec<Motion>) {
    let mut head = Position { col: 0, row: 0 };
    let mut tail = Position { col: 0, row: 0 };
    let mut tail_positions = HashSet::from([tail.clone()]);

    for motion in motions {
        use Motion::*;
        match motion {
            Up => {
                head.row += 1;
            }
            Down => {
                head.row -= 1;
            }
            Right => {
                head.col += 1;
            }
            Left => {
                head.col -= 1;
            }
        }

        if !is_connected(&head, &tail) {
            let mut new_tail = head.clone();
            match motion {
                Up => {
                    new_tail.row -= 1;
                }
                Down => {
                    new_tail.row += 1;
                }
                Right => {
                    new_tail.col -= 1;
                }
                Left => {
                    new_tail.col += 1;
                }
            }
            tail = new_tail;
            tail_positions.insert(tail.clone());
        }
    }

    let count = tail_positions.len();
    println!("Part 1: {count}");
}

fn part2(motions: Vec<Motion>) {
    let mut rope = vec![Position { col: 0, row: 0 }; 10];
    let mut tail_positions = HashSet::from([rope.last().unwrap().clone()]);

    for motion in motions {
        use Motion::*;
        match motion {
            Up => {
                rope[0].row += 1;
            }
            Down => {
                rope[0].row -= 1;
            }
            Right => {
                rope[0].col += 1;
            }
            Left => {
                rope[0].col -= 1;
            }
        }

        let mut rope_windows = rope.windows_mut::<2>();
        while let Some([ref mut head, ref mut tail]) = rope_windows.next() {
            if !is_connected(head, tail) {
                if head.col == tail.col {
                    if head.row > tail.row {
                        tail.row += 1;
                    } else {
                        tail.row -= 1;
                    }
                } else if head.row == tail.row {
                    if head.col > tail.col {
                        tail.col += 1;
                    } else {
                        tail.col -= 1;
                    }
                } else {
                    let col_range = (head.col - 1)..=(head.col + 1);
                    let row_range = (head.row - 1)..=(head.row + 1);

                    let mut head_3x3 = Vec::new();
                    for col in col_range {
                        for row in row_range.clone() {
                            head_3x3.push(Position { col, row });
                        }
                    }

                    let col_range = (tail.col - 1)..=(tail.col + 1);
                    let row_range = (tail.row - 1)..=(tail.row + 1);

                    let mut maybe_new_tail = Vec::new();
                    for col in col_range {
                        for row in row_range.clone() {
                            let col = Position { col, row };
                            if head_3x3.contains(&col) {
                                maybe_new_tail.push(col);
                            }
                        }
                    }

                    match maybe_new_tail.len() {
                        2 => {
                            let new_head_cross_positions = [
                                Position {
                                    col: head.col - 1,
                                    row: head.row,
                                },
                                Position {
                                    col: head.col + 1,
                                    row: head.row,
                                },
                                Position {
                                    col: head.col,
                                    row: head.row - 1,
                                },
                                Position {
                                    col: head.col,
                                    row: head.row + 1,
                                },
                            ];

                            let next = maybe_new_tail
                                .iter()
                                .find(|pos| new_head_cross_positions.contains(pos))
                                .unwrap();

                            *tail = next.clone();
                        }
                        1 => {
                            *tail = maybe_new_tail[0].clone();
                        }
                        _ => panic!("unknown tail length"),
                    }
                }
            }
        }

        tail_positions.insert(rope.last().unwrap().clone());
    }

    let count = tail_positions.len();
    println!("Part 2: {count}");
}

fn is_connected(head: &Position, tail: &Position) -> bool {
    let col_range = (head.col - 1)..=(head.col + 1);
    let row_range = (head.row - 1)..=(head.row + 1);

    for col in col_range {
        for row in row_range.clone() {
            let pos = Position { col, row };
            if pos == *tail {
                return true;
            }
        }
    }

    false
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    col: isize,
    row: isize,
}

#[derive(Debug, Clone)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Motion::*;
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(()),
        }
    }
}
