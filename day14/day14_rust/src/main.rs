use std::str::FromStr;

fn main() {
    let content = std::fs::read_to_string("../data").unwrap();
    let rocks = parse_rocks(&content);

    let part1 = part1(&rocks);
    println!("Part 1: {}", part1);
}

fn parse_rocks(content: &str) -> Vec<RockStructure> {
    content
        .lines()
        .map(|l| l.parse::<RockStructure>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(rocks: &[RockStructure]) -> usize {
    let sand_start = Position { x: 500, y: 0 };

    let min_x = rocks
        .iter()
        .map(|r| r.coordinates.iter().map(|c| c.x).min().unwrap())
        .min()
        .unwrap();

    let max_x = rocks
        .iter()
        .map(|r| r.coordinates.iter().map(|c| c.x).max().unwrap())
        .max()
        .unwrap();

    let max_y = rocks
        .iter()
        .map(|r| r.coordinates.iter().map(|c| c.y).max().unwrap())
        .max()
        .unwrap();

    let box_ = Box {
        min_x,
        min_y: 0,
        max_x,
        max_y,
    };

    dbg!(&box_);

    let mut sands = Vec::new();
    'outer: loop {
        let mut sand = sand_start.clone();

        let mut down_occupied =
            rocks.iter().any(|r| r.contains(&sand.get_down())) || sands.contains(&sand.get_down());
        let mut down_left_occupied = rocks.iter().any(|r| r.contains(&sand.get_down_left()))
            || sands.contains(&sand.get_down_left());
        let mut down_right_occupied = rocks.iter().any(|r| r.contains(&sand.get_down_right()))
            || sands.contains(&sand.get_down_right());

        'inner: loop {
            if down_occupied && down_left_occupied && down_right_occupied {
                if box_.contains(&sand) {
                    sands.push(sand.clone());
                } else {
                    break 'outer;
                }
            }

            if !down_occupied {
                sand.move_down();
            } else if !down_left_occupied {
                sand.move_down_left();
            } else if !down_right_occupied {
                sand.move_down_right();
            } else {
                break 'inner;
            }

            if !box_.contains(&sand) {
                break 'outer;
            }

            down_occupied = rocks.iter().any(|r| r.contains(&sand.get_down()))
                || sands.contains(&sand.get_down());
            down_left_occupied = rocks.iter().any(|r| r.contains(&sand.get_down_left()))
                || sands.contains(&sand.get_down_left());
            down_right_occupied = rocks.iter().any(|r| r.contains(&sand.get_down_right()))
                || sands.contains(&sand.get_down_right());
        }
    }

    draw_board(&box_, rocks, &sands);

    sands.len()
}

fn draw_board(box_: &Box, rocks: &[RockStructure], sands: &Vec<Position>) {
    for y in box_.min_y..=box_.max_y {
        for x in box_.min_x..=box_.max_x {
            let position = Position { x, y };
            if rocks.iter().any(|r| r.contains(&position)) {
                print!("#");
            } else if sands.contains(&position) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug)]
struct Box {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Box {
    fn contains(&self, position: &Position) -> bool {
        position.x >= self.min_x
            && position.x <= self.max_x
            && position.y >= self.min_y
            && position.y <= self.max_y
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RockStructure {
    coordinates: Vec<Position>,
}

impl RockStructure {
    fn contains(&self, position: &Position) -> bool {
        self.coordinates.contains(position)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn x_gap(&self, other: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut x = self.x;
        while x != other.x {
            if x < other.x {
                x += 1;
            } else {
                x -= 1;
            }
            positions.push(Position { x, y: self.y });
        }

        positions
    }

    fn y_gap(&self, other: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut y = self.y;
        while y != other.y {
            if y < other.y {
                y += 1;
            } else {
                y -= 1;
            }
            positions.push(Position { x: self.x, y });
        }

        positions
    }

    fn move_down(&mut self) {
        self.y += 1;
    }

    fn get_down(&self) -> Position {
        let mut position = self.clone();
        position.move_down();
        position
    }

    fn move_down_left(&mut self) {
        self.y += 1;
        self.x -= 1;
    }

    fn get_down_left(&self) -> Position {
        let mut position = self.clone();
        position.move_down_left();
        position
    }

    fn move_down_right(&mut self) {
        self.y += 1;
        self.x += 1;
    }

    fn get_down_right(&self) -> Position {
        let mut position = self.clone();
        position.move_down_right();
        position
    }
}

impl FromStr for RockStructure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates = s
            .split(" -> ")
            .map(|cord| {
                let mut cord = cord.split(",");
                let x = cord.next().unwrap().parse::<usize>().unwrap();
                let y = cord.next().unwrap().parse::<usize>().unwrap();
                Position { x, y }
            })
            .fold(Vec::<Position>::new(), |mut acc, cord| match acc.last() {
                Some(last) => {
                    let x_gap = last.x_gap(&cord);
                    let y_gap = last.y_gap(&cord);
                    acc.extend(x_gap);
                    acc.extend(y_gap);
                    acc
                }
                None => {
                    acc.push(cord);
                    acc
                }
            });

        Ok(Self { coordinates })
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_rocks, Position, RockStructure};

    const TEST_INPUT: &str = include_str!("../../test");
    #[test]
    fn parsing_test() {
        let rocks = parse_rocks(TEST_INPUT);

        assert_eq!(
            rocks,
            vec![
                RockStructure {
                    coordinates: vec![
                        Position { x: 498, y: 4 },
                        Position { x: 498, y: 5 },
                        Position { x: 498, y: 6 }, // turn
                        Position { x: 497, y: 6 },
                        Position { x: 496, y: 6 },
                    ]
                },
                RockStructure {
                    coordinates: vec![
                        Position { x: 503, y: 4 },
                        Position { x: 502, y: 4 },
                        Position { x: 502, y: 5 }, // turn
                        Position { x: 502, y: 6 },
                        Position { x: 502, y: 7 },
                        Position { x: 502, y: 8 },
                        Position { x: 502, y: 9 }, // turn
                        Position { x: 501, y: 9 },
                        Position { x: 500, y: 9 },
                        Position { x: 499, y: 9 },
                        Position { x: 498, y: 9 },
                        Position { x: 497, y: 9 },
                        Position { x: 496, y: 9 },
                        Position { x: 495, y: 9 },
                        Position { x: 494, y: 9 },
                    ]
                }
            ]
        );
    }
}
