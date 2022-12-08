fn main() {
    let input = std::fs::read_to_string("../data")
        .unwrap()
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|n| n.parse::<u8>().ok())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let mut trees = Vec::with_capacity(input.len() * input[0].len());

    for (row, line) in input.iter().enumerate() {
        for (col, tree) in line.iter().enumerate() {
            let mut top_to_edge = Vec::new();
            for i in 1..=row {
                if let Some(t) = input.get(row - i).and_then(|r| r.get(col)) {
                    top_to_edge.push(*t);
                }
            }
            let mut bottom_to_edge = Vec::new();
            for i in 1..=input.len() - row {
                if let Some(b) = input.get(row + i).and_then(|r| r.get(col)) {
                    bottom_to_edge.push(*b);
                }
            }
            let mut left_to_edge = Vec::new();
            for i in 1..=col {
                if let Some(l) = input.get(row).and_then(|r| r.get(col - i)) {
                    left_to_edge.push(*l);
                }
            }
            let mut right_to_edge = Vec::new();
            for i in 1..=input[0].len() - col {
                if let Some(r) = input.get(row).and_then(|r| r.get(col + i)) {
                    right_to_edge.push(*r);
                }
            }

            trees.push(Tree {
                value: *tree,
                top_to_edge,
                bottom_to_edge,
                right_to_edge,
                left_to_edge,
            });
        }
    }

    let visible_trees = trees.iter().filter(|t| t.is_visible()).count();
    let max_scenic_score = trees.iter().map(|t| t.scenic_score()).max();

    println!("Part 1: {}", visible_trees);
    println!("Part 2: {:?}", max_scenic_score);
}

#[derive(Debug)]
struct Tree {
    value: u8,
    top_to_edge: Vec<u8>,
    left_to_edge: Vec<u8>,
    right_to_edge: Vec<u8>,
    bottom_to_edge: Vec<u8>,
}

impl Tree {
    fn is_visible(&self) -> bool {
        let visible_on_top = self.top_to_edge.iter().all(|t| *t < self.value);
        let visible_on_bottom = self.bottom_to_edge.iter().all(|t| *t < self.value);
        let visible_on_left = self.left_to_edge.iter().all(|t| *t < self.value);
        let visible_on_right = self.right_to_edge.iter().all(|t| *t < self.value);

        visible_on_top || visible_on_bottom || visible_on_left || visible_on_right
    }

    fn scenic_score(&self) -> usize {
        let func = |(acc, blocked): (usize, bool), &t: &u8| {
            if blocked {
                (acc, blocked)
            } else {
                let acc = acc + 1;
                match t.cmp(&self.value) {
                    std::cmp::Ordering::Less => (acc, false),
                    std::cmp::Ordering::Equal => (acc, true),
                    std::cmp::Ordering::Greater => (acc, true),
                }
            }
        };

        let (top_view, _) = self.top_to_edge.iter().fold((0, false), func);
        let (bottom_view, _) = self.bottom_to_edge.iter().fold((0, false), func);
        let (left_view, _) = self.left_to_edge.iter().fold((0, false), func);
        let (right_view, _) = self.right_to_edge.iter().fold((0, false), func);

        top_view * bottom_view * left_view * right_view
    }
}
