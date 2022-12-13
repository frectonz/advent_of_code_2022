fn main() {
    let lines = std::fs::read_to_string("../test")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = lines.iter().position(|l| l.contains(&'@')).unwrap();
    dbg!(lines);
}
