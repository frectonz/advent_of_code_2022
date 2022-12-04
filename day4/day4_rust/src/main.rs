use std::str::FromStr;

fn main() {
    let content = std::fs::read_to_string("../data").unwrap();

    let (part1, part2) = content
        .lines()
        .map(|line| {
            let mut line = line.split(",");

            let first: Range = line.next().unwrap().parse().unwrap();
            let second: Range = line.next().unwrap().parse().unwrap();

            (first, second)
        })
        .fold((0, 0), |(part1_acc, part2_acc), (first, second)| {
            let part1 = if first.contains(&second) {
                part1_acc + 1
            } else if second.contains(&first) {
                part1_acc + 1
            } else {
                part1_acc
            };

            let part2 = if first.overlaps(&second) {
                part2_acc + 1
            } else if second.overlaps(&first) {
                part2_acc + 1
            } else {
                part2_acc
            };

            (part1, part2)
        });

    dbg!(part1, part2);
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, range: &Range) -> bool {
        self.start <= range.start && self.end >= range.end
    }

    fn overlaps(&self, range: &Range) -> bool {
        self.start <= range.start && self.end >= range.start
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("-");

        let start = s.next().unwrap().parse().unwrap();
        let end = s.next().unwrap().parse().unwrap();

        Ok(Range { start, end })
    }
}
