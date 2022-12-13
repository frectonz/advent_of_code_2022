use std::{cmp::Ordering, fmt::Display, str::FromStr};

fn main() {
    let content = std::fs::read_to_string("../data").unwrap();

    let part1_answer = part1(&content);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&content);
    println!("Part 2: {}", part2_answer);
}

fn part1(content: &str) -> usize {
    let pairs = content.split("\n\n");

    let pairs = pairs
        .into_iter()
        .map(|pair| pair.parse::<Pair>().unwrap())
        .map(|pair| pair.compare())
        .collect::<Vec<_>>();

    pairs
        .iter()
        .enumerate()
        .filter(|(_, ord)| ord == &&Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

fn part2(content: &str) -> usize {
    let pairs = content.split("\n\n");

    let divider_packets = [
        "[[2]]".parse::<List>().unwrap(),
        "[[6]]".parse::<List>().unwrap(),
    ];
    let mut pairs = pairs
        .into_iter()
        .map(|pair| pair.parse::<Pair>().unwrap())
        .flat_map(|pair| [pair.first, pair.second])
        .chain(divider_packets.clone())
        .collect::<Vec<_>>();

    pairs.sort();

    let first_divider_index = pairs
        .iter()
        .enumerate()
        .find(|(_, list)| list == &&divider_packets[0])
        .map(|(i, _)| i + 1)
        .unwrap();

    let second_divider_index = pairs
        .iter()
        .enumerate()
        .find(|(_, list)| list == &&divider_packets[1])
        .map(|(i, _)| i + 1)
        .unwrap();

    first_divider_index * second_divider_index
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum List {
    Elements(Vec<List>),
    Element(u32),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        use List::*;

        match (self, other) {
            (Element(a), Element(b)) => a.cmp(b),
            (Elements(a), Elements(b)) => a.cmp(b),
            (Element(a), Elements(b)) => Elements(vec![Element(*a)]).cmp(&Elements(b.clone())),
            (Elements(a), Element(b)) => Elements(a.clone()).cmp(&Elements(vec![Element(*b)])),
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use List::*;
        match self {
            Elements(elements) => {
                let elements = elements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", elements)
            }
            Element(element) => write!(f, "{}", element),
        }
    }
}

fn parse_list(input: &str) -> nom::IResult<&str, List> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace0},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::{delimited, preceded},
    };

    let element = map_res(digit1, |s: &str| s.parse::<u32>());
    let elements = delimited(tag("["), separated_list0(tag(","), parse_list), tag("]"));
    let list = alt((map(element, List::Element), map(elements, List::Elements)));

    preceded(multispace0, list)(input)
}

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list = parse_list(s).map(|(_, list)| list).map_err(|_| ())?;
        Ok(list)
    }
}

#[derive(Debug)]
struct Pair {
    first: List,
    second: List,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').collect::<Vec<_>>();
        let first = lines[0].parse::<List>()?;
        let second = lines[1].parse::<List>()?;

        Ok(Pair { first, second })
    }
}

impl Pair {
    fn compare(self) -> Ordering {
        self.first.cmp(&self.second)
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_test() {
        let input = include_str!("../../test");
        assert_eq!(part1(input), 13);

        let input = include_str!("../../data");
        assert_eq!(part1(input), 5808);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../../test");
        assert_eq!(part2(input), 140);

        let input = include_str!("../../data");
        assert_eq!(part2(input), 22_713);
    }
}
