use std::{collections::HashMap, str::FromStr};

fn main() {
    let content = std::fs::read_to_string("../data").unwrap();

    let lines = content
        .lines()
        .map(|line| line.parse::<Line>())
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let dirs = make_dir_list_from_lines(lines);
    dbg!(&dirs);
    let dir_sizes = make_directory_size_map(dirs);

    let part1_result: usize = dir_sizes
        .iter()
        .filter(|(_, size)| **size <= 100_000)
        .map(|(_, size)| *size)
        .sum();

    dbg!(part1_result);

    let root_size = dir_sizes.get("/").unwrap();
    let unused_space = 70000000 - root_size;

    let smallest_dir_with_enough_space = dir_sizes
        .iter()
        .filter(|(_, size)| unused_space + **size >= 30000000)
        .min_by_key(|(_, size)| *size)
        .unwrap();

    dbg!(smallest_dir_with_enough_space);
}

#[derive(Debug)]
enum Line {
    Command(Command),
    LsOutput(LsOutput),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Command>()
            .map(Line::Command)
            .or_else(|_| s.parse::<LsOutput>().map(Line::LsOutput))
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        if parts.next() != Some("$") {
            return Err(());
        }
        let command = parts.next().unwrap();
        match command {
            "cd" => {
                let path = parts.next().unwrap();
                Ok(Command::Cd(path.to_string()))
            }
            "ls" => Ok(Command::Ls),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum LsOutput {
    Directory(String),
    File { name: String, size: usize },
}

impl FromStr for LsOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("dir") => {
                let name = parts.next().unwrap();
                Ok(LsOutput::Directory(name.to_string()))
            }
            Some(size) => {
                if let Ok(size) = size.parse::<usize>() {
                    let file = parts.next().unwrap();
                    Ok(LsOutput::File {
                        name: file.to_string(),
                        size,
                    })
                } else {
                    Err(())
                }
            }
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    contents: Vec<DirEntry>,
}

#[derive(Debug, Clone)]
enum DirEntry {
    Dir(String),
    File(usize, String),
}

impl DirEntry {
    fn get_dir_name(&self) -> Option<&str> {
        match self {
            DirEntry::Dir(name) => Some(name),
            DirEntry::File(_, _) => None,
        }
    }
}

fn make_directory_size_map(dirs: Vec<Directory>) -> HashMap<String, usize> {
    let mut dir_size_map = HashMap::new();
    for dir in dirs.clone() {
        let size = get_size(dir.clone(), dirs.clone(), &mut dir_size_map);
        dir_size_map.insert(dir.name, size);
    }
    dir_size_map
}

fn get_size(
    dir: Directory,
    dirs: Vec<Directory>,
    dir_size_map: &mut HashMap<String, usize>,
) -> usize {
    dir.contents
        .iter()
        .map(|entry| match entry {
            DirEntry::Dir(name) => match dir_size_map.get(name) {
                Some(size) => *size,
                None => {
                    let dir = dirs
                        .clone()
                        .into_iter()
                        .find(|d| d.name == *name)
                        .expect("Directory not found");

                    get_size(dir, dirs.clone(), dir_size_map)
                }
            },
            DirEntry::File(size, _) => *size,
        })
        .sum()
}

fn make_dir_list_from_lines(lines: Vec<Line>) -> Vec<Directory> {
    let mut dir_stack = Vec::new();
    let mut dirs = Vec::new();
    for line in lines {
        match line {
            Line::Command(Command::Cd(path)) => handle_cd(&mut dir_stack, &mut dirs, &path),
            Line::LsOutput(ls_output) => handle_ls_output(ls_output, &mut dir_stack),
            _ => {}
        }
    }
    dirs.append(&mut dir_stack);
    dirs
}

fn handle_cd(dir_stack: &mut Vec<Directory>, dirs: &mut Vec<Directory>, path: &str) {
    if path == "/" {
        let root = Directory {
            name: "/".into(),
            contents: Vec::new(),
        };
        dir_stack.push(root);
    } else if path == ".." {
        if let Some(dir) = dir_stack.pop() {
            dirs.push(dir);
        }
    } else {
        match dir_stack.last() {
            Some(current_dir) => {
                let path = format!("{}/{}", current_dir.name, path);
                let found = current_dir
                    .contents
                    .iter()
                    .filter_map(|e| e.get_dir_name())
                    .any(|name| name == path);

                if found {
                    let new_dir = Directory {
                        name: path,
                        contents: Vec::new(),
                    };
                    dir_stack.push(new_dir);
                }
            }
            None => {
                let new_dir = Directory {
                    name: format!("//{}", path),
                    contents: Vec::new(),
                };
                dir_stack.push(new_dir);
            }
        };
    }
}

fn handle_ls_output(ls_output: LsOutput, dir_stack: &mut [Directory]) {
    match ls_output {
        LsOutput::Directory(name) => {
            let current_dir = dir_stack.last_mut().unwrap();
            let new_dir = DirEntry::Dir(format!("{}/{}", current_dir.name, name));
            current_dir.contents.push(new_dir);
        }
        LsOutput::File { name, size } => {
            let new_file = DirEntry::File(size, name);
            let current_dir = dir_stack.last_mut().unwrap();
            current_dir.contents.push(new_file);
        }
    }
}
