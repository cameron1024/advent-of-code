const INPUT: &str = include_str!("../inputs/day_2");

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

fn main() {
    let result = part_1();
    println!("{result}");
    let result = part_2();
    println!("{result}");
}

fn part_1() -> i32 {
    INPUT
        .lines()
        .map(parse_line)
        .map(|(their_shape, my_shape)| {
            let outcome = outcome(my_shape, their_shape);
            score(my_shape, outcome)
        })
        .sum()
}

fn part_2() -> i32 {
    INPUT.lines().map(parse_line_2).map(|(their_shape, outcome)| {
        let my_shape = what_to_play(their_shape, outcome);
        score(my_shape, outcome)
    }).sum()
}

fn what_to_play(their_shape: Shape, outcome: Outcome) -> Shape {
    use Outcome::*;
    use Shape::*;

    match (their_shape, outcome) {
        (Rock, Loss) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,

        (Paper, Loss) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,

        (Scissors, Loss) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    }
}

fn parse_line(line: &str) -> (Shape, Shape) {
    let mut letters = line.split_whitespace();
    let their_shape = match letters.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => unreachable!(),
    };

    let my_shape = match letters.next().unwrap() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => unreachable!(),
    };

    (their_shape, my_shape)
}

fn parse_line_2(line: &str) -> (Shape, Outcome) {
    let mut letters = line.split_whitespace();
    let their_shape = match letters.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => unreachable!(),
    };

    let outcome = match letters.next().unwrap() {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!(),
    };

    (their_shape, outcome)
}

fn outcome(my_shape: Shape, their_shape: Shape) -> Outcome {
    use Shape::*;
    match (my_shape, their_shape) {
        (Rock, Rock) => Outcome::Draw,
        (Rock, Paper) => Outcome::Loss,
        (Rock, Scissors) => Outcome::Win,

        (Paper, Rock) => Outcome::Win,
        (Paper, Paper) => Outcome::Draw,
        (Paper, Scissors) => Outcome::Loss,

        (Scissors, Rock) => Outcome::Loss,
        (Scissors, Paper) => Outcome::Win,
        (Scissors, Scissors) => Outcome::Draw,
    }
}

fn score(my_shape: Shape, outcome: Outcome) -> i32 {
    let shape_score = match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let outcome_score = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    shape_score + outcome_score
}
