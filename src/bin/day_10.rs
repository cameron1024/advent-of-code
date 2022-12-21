const INPUT: &str = include_str!("../inputs/day_10");

fn main() {
    let result = part_1(INPUT);
    println!("{result}");
}

fn part_1(s: &str) -> i64 {
    let mut sum = 0;
    let mut vm = Vm { x: 1 };
    let iter = s
        .lines()
        .map(Instr::from_str)
        .flat_map(|instr| match instr {
            Instr::Noop => vec![Instr::Noop],
            Instr::Addx(i) => vec![Instr::Noop, Instr::Addx(i)],
        })
        .enumerate();

    let indexes = [20, 60, 100, 140, 180, 220];

    for (index, instr) in iter {
        if indexes.contains(&index) {
            sum += (index as i64) * vm.x;
        }
        vm.apply(instr);
    }

    sum
}

struct Vm {
    x: i64,
}

impl Vm {
    fn apply(&mut self, instr: Instr) {
        match instr {
            Instr::Noop => {}
            Instr::Addx(i) => self.x += i,
        }
    }
}

#[derive(Clone, Copy)]
enum Instr {
    Addx(i64),
    Noop,
}

impl Instr {
    fn from_str(s: &str) -> Self {
        match s.split_once(' ') {
            None => Self::Noop,
            Some(("addx", n)) => Self::Addx(n.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}
