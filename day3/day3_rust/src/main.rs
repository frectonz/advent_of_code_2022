fn main() {
    part1();
    part2();
}

fn part1() {
    let content = std::fs::read_to_string("../data").unwrap();
    let sum: usize = content
        .lines()
        .map(|line| {
            let half_point = line.len() / 2;

            let first_half = &line[..half_point];
            let second_half = &line[half_point..];

            for c in first_half.chars() {
                if second_half.contains(c) {
                    return char_to_priority(&c);
                }
            }
            return 0;
        })
        .sum();

    dbg!(sum);
}

fn part2() {
    let content = std::fs::read_to_string("../data").unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..lines.len() / 3 {
        let i = i * 3;

        let one = lines[i];
        let two = lines[i + 1];
        let three = lines[i + 2];

        for c in one.chars() {
            if two.contains(c) && three.contains(c) {
                sum += char_to_priority(&c);
                break;
            }
        }
    }

    dbg!(sum);
}

fn char_to_priority(c: &char) -> usize {
    if c.is_uppercase() {
        *c as usize - 38
    } else {
        *c as usize - 96
    }
}
