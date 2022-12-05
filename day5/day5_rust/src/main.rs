fn main() {
    let content = std::fs::read_to_string("../data").unwrap();
    let mut content = content.split("\n\n");

    let stack = content.next().unwrap();
    let mut lines = stack.lines().collect::<Vec<_>>();

    let num_of_stacks = lines
        .last()
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .count();

    lines.remove(lines.len() - 1);

    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(num_of_stacks);

    for _ in 0..num_of_stacks {
        stacks.push(Vec::new());
    }

    for line in lines {
        let words = line.split("_").enumerate().collect::<Vec<_>>();

        for (i, word) in words {
            if word != "   " {
                stacks[i].push(word[1..word.len() - 1].to_string());
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let procedures = content.next().unwrap();
    let procedures = procedures
        .lines()
        .map(|line| {
            let line = line.split(" ").collect::<Vec<_>>();
            let items = line[1].parse::<usize>().unwrap();
            let from = line[3].parse::<usize>().unwrap();
            let to = line[5].parse::<usize>().unwrap();

            Procedure { items, from, to }
        })
        .collect::<Vec<_>>();

    // crate_mover_9000
    // for procedure in procedures {
    //     for _ in 0..procedure.items {
    //         let from = stacks[procedure.from - 1].pop().unwrap();
    //         stacks[procedure.to - 1].push(from);
    //     }
    // }

    // crate_mover_9001
    for procedure in procedures {
        let from = &mut stacks[procedure.from - 1];
        let items = from.split_off(from.len() - procedure.items);

        let to = &mut stacks[procedure.to - 1];
        to.append(&mut items.clone());
    }

    let top = stacks.iter().map(|stack| stack.last().unwrap().as_str());
    let top = top.collect::<String>();

    dbg!(top);
}

#[derive(Debug)]
struct Procedure {
    items: usize,
    from: usize,
    to: usize,
}
