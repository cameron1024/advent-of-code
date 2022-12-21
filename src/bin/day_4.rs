use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/day_4");

fn main() {
    let result = part_1();
    println!("{result}");
    let result = part_2();
    println!("{result}");
}

#[derive(Clone, Copy)]
struct Line {
    start_1: i32,
    end_1: i32,
    start_2: i32,
    end_2: i32,
}

impl Line {
    fn any_overlap(&self) -> bool {
        if self.one_contains_other() {
            return true;
        }

        let first = (self.start_1..=self.end_1).collect::<HashSet<_>>();
        let second = (self.start_2..=self.end_2).collect::<HashSet<_>>();

        first.iter().any(|i| second.contains(i))
    }

    fn one_contains_other(&self) -> bool {
        self.first_contains_second() || self.second_contains_first()
    }

    fn first_contains_second(&self) -> bool {
        let Self {
            start_1,
            end_1,
            start_2,
            end_2,
        } = *self;
        start_1 <= start_2 && end_1 >= end_2
    }

    fn second_contains_first(&self) -> bool {
        let Self {
            start_1,
            end_1,
            start_2,
            end_2,
        } = *self;
        start_2 <= start_1 && end_2 >= end_1
    }
}

fn parse_line(s: &str) -> Line {
    let (start_1, s) = s.split_once('-').unwrap();
    let (end_1, s) = s.split_once(',').unwrap();
    let (start_2, end_2) = s.split_once('-').unwrap();

    let start_1 = start_1.parse().unwrap();
    let end_1 = end_1.parse().unwrap();
    let start_2 = start_2.parse().unwrap();
    let end_2 = end_2.parse().unwrap();

    Line {
        start_1,
        end_1,
        start_2,
        end_2,
    }
}

fn part_1() -> usize {
    INPUT
        .lines()
        .map(parse_line)
        .filter(|l| l.one_contains_other())
        .count()
}

fn part_2() -> usize {
    INPUT
        .lines()
        .map(parse_line)
        .filter(|l| l.any_overlap())
        .count()
}
