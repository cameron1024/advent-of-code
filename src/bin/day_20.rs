use std::{collections::VecDeque, str::FromStr};

const INPUT: &str = include_str!("../inputs/day_20");

fn main() {}

struct File {
    contents: Vec<i32>,
}

impl File {
    fn from_str(s: &str) -> Self {
        let contents = s.lines().map(|s| s.parse().unwrap()).collect();

        Self { contents }
    }

    fn cycle(&mut self, index: usize, amount: isize) {
        let len = self.contents.len();
        match amount {
            0 => {}
            1.. => {
                let new_index = (index + 1) % len;
                self.contents.swap(index, new_index);
                self.cycle(new_index, amount - 1);
            }
            _ => {
                let new_index = (index + len - 1) % len;
                self.contents.swap(index, new_index);
                self.cycle(new_index, amount + 1);
            }
        }
    }

    fn cycle_all(&mut self) {
        let contents_clone = self.contents.clone();

        for 
    }
}
