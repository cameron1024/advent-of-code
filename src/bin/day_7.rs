use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_7");

fn main() {}

enum Line<'a> {
    Cd(&'a str),
    Ls,
    Dir(&'a str),
    File(usize, &'a str),
}

fn parse_line(s: &str) -> Line {
    let (head, tail) = s.split_once(' ').unwrap();

    match head {
        "$" => match tail {
            "ls" => Line::Ls,
            _ => Line::Cd(tail.split_once(' ').unwrap().1),
        },
        "dir" => Line::Dir(tail),
        _ => Line::File(head.parse().unwrap(), tail),
    }
}

enum FsEntry<'a> {
    Dir(&'a str, HashMap<&'a str, FsEntry<'a>>),
    File(usize),
}

fn build_fs(s: &str) -> FsEntry {
    let mut root = FsEntry::Dir("/", HashMap::new());
    let mut current_dir = "";
;

    for line in s.lines() {
    }

    todo!()
}
