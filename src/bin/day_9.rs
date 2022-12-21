use std::collections::HashSet;

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: Direction,
    distance: i64,
}

fn parse_motions(s: &str) -> impl Iterator<Item = Motion> + '_ {
    s.lines().map(|line| {
        let (direction, distance) = line.split_once(' ').unwrap();

        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };

        let distance = distance.parse().unwrap();

        Motion {
            direction,
            distance,
        }
    })
}

struct Grid {
    head: Coord,
    tail: Coord,
    visited: HashSet<Coord>,
}

impl Grid {
    fn new() -> Self {
        Self {
            head: Coord { x: 0, y: 0 },
            tail: Coord { x: 0, y: 0 },
            visited: HashSet::new(),
        }
    }

    fn tail_needs_to_move(&self) -> bool {
        let dx = self.head.x.abs_diff(self.tail.x);
        let dy = self.head.y.abs_diff(self.tail.y);

        dx > 1 || dy > 1
    }

    fn apply_motion(
        &mut self,
        Motion {
            direction,
            distance,
        }: Motion,
    ) {
        match direction {
            Direction::Up => self.head.y += distance,
            Direction::Down => self.head.y -= distance,
            Direction::Left => self.head.x -= distance,
            Direction::Right => self.head.x += distance,
        };

        if self.tail_needs_to_move() {
            self.move_tail();
            self.visited.insert(self.tail);
        }
    }

    fn move_tail(&mut self) {
        while self.tail_needs_to_move() {
            if self.head.x - self.tail.x >= 2 {
                self.tail.x += 1;
            }
        }
    }
}

fn part_1(s: &str) -> usize { 0 }
