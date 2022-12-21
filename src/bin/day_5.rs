const INPUT: &str = include_str!("../inputs/day_5");

fn main() {
    let result = part_1();
    println!("{result}");
}

fn part_1() -> String {
    let (mut stacks, instrs) = parse(INPUT, 9);
    stacks.apply_all(&instrs);
    stacks.tops()
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Instruction {
    mv: usize,
    from: usize,
    to: usize,
}

fn parse(s: &str, len: usize) -> (Stacks, Vec<Instruction>) {
    let (stacks, instructions) = s.split_once("\n\n").unwrap();

    let stacks = parse_header(stacks, len);
    let instructions = parse_instructions(instructions);

    (stacks, instructions)
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let _move = words.next().unwrap();
            let mv = words.next().unwrap().parse().unwrap();
            let _from = words.next().unwrap();
            let from = words.next().unwrap().parse().unwrap();
            let _to = words.next().unwrap();
            let to = words.next().unwrap().parse().unwrap();

            Instruction { mv, from, to }
        })
        .collect()
}

fn parse_header(s: &str, len: usize) -> Stacks {
    let mut stacks = vec![vec![]; len];

    for line in s.lines() {
        for i in 0..len {
            let c = line.as_bytes()[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c as char);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    Stacks { stacks }
}

impl Stacks {
    fn apply_all(&mut self, instrs: &[Instruction]) {
        for instr in instrs {
            self.apply(instr);
        }
    }

    fn apply(&mut self, instr: &Instruction) {
        println!("{self:?}");
        println!("applying: {instr:?}");
        let Instruction { mv, from, to } = *instr;
        let vec_from = self.stacks.get_mut(from - 1).unwrap();
        let new_len = vec_from.len() - mv;
        let mut elems = vec_from.drain(new_len..).collect::<Vec<_>>();
        let vec_to = self.stacks.get_mut(to - 1).unwrap();
        vec_to.extend(elems);
    }

    fn tops(&self) -> String {
        self.stacks.iter().map(|v| v.last().unwrap()).collect()
    }
}
