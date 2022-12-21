#![feature(box_patterns)]
use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/day_21");

fn main() {
    let result = part_1();
    println!("{result}");
    let result = part_2();
    println!("{result}");
}

fn part_1() -> i64 {
    let monkeys = Monkeys::from_str(INPUT);
    monkeys.eval("root")
}

fn part_2() -> i64 {
    let monkeys = Monkeys::from_str(INPUT);
    let Monkey::Add(a, b) = monkeys.monkeys.get("root").unwrap() else { panic!() };
    let a = monkeys.to_expr(a);
    let b = monkeys.to_expr(b);

    match (a.force_eval(), b.force_eval()) {
        (None, Some(i)) => solve(a, i),
        (Some(i), None) => solve(b, i),
        _ => unreachable!(),
    }
}

struct Monkeys<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
}

enum Monkey<'a> {
    Const(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> Monkeys<'a> {
    fn eval(&self, name: &str) -> i64 {
        match self.monkeys.get(name) {
            None => panic!("unknown monkey: {name}"),
            Some(Monkey::Const(i)) => *i,
            Some(Monkey::Add(a, b)) => self.eval(a) + self.eval(b),
            Some(Monkey::Sub(a, b)) => self.eval(a) - self.eval(b),
            Some(Monkey::Mul(a, b)) => self.eval(a) * self.eval(b),
            Some(Monkey::Div(a, b)) => self.eval(a) / self.eval(b),
        }
    }

    fn from_str(s: &'a str) -> Self {
        let monkeys = s.lines().map(parse_line).collect();

        Self { monkeys }
    }

    fn to_expr(&self, name: &str) -> Expr {
        use Monkey::*;

        if name == "humn" {
            return Expr::Human;
        }

        match self.monkeys.get(name) {
            None => panic!("unknown monkey: {name}"),
            Some(Const(i)) => Expr::Const(*i),
            Some(Add(a, b)) => Expr::Add(Box::new(self.to_expr(a)), Box::new(self.to_expr(b))),
            Some(Sub(a, b)) => Expr::Sub(Box::new(self.to_expr(a)), Box::new(self.to_expr(b))),
            Some(Mul(a, b)) => Expr::Mul(Box::new(self.to_expr(a)), Box::new(self.to_expr(b))),
            Some(Div(a, b)) => Expr::Div(Box::new(self.to_expr(a)), Box::new(self.to_expr(b))),
        }
    }
}

fn parse_line<'a>(s: &'a str) -> (&'a str, Monkey<'a>) {
    let (name, s) = s.split_once(": ").unwrap();

    if let Ok(i) = s.parse() {
        return (name, Monkey::Const(i));
    }

    let mut words = s.split(' ');
    let lhs = words.next().unwrap();
    let op = words.next().unwrap();
    let rhs = words.next().unwrap();

    match op {
        "+" => (name, Monkey::Add(lhs, rhs)),
        "-" => (name, Monkey::Sub(lhs, rhs)),
        "*" => (name, Monkey::Mul(lhs, rhs)),
        "/" => (name, Monkey::Div(lhs, rhs)),
        s => panic!("unknown op: {s}"),
    }
}

enum Expr {
    Human,
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn contains_human(&self) -> bool {
        match self {
            Expr::Human => true,
            Expr::Const(_) => false,
            Expr::Add(box a, box b) => a.contains_human() || b.contains_human(),
            Expr::Sub(box a, box b) => a.contains_human() || b.contains_human(),
            Expr::Mul(box a, box b) => a.contains_human() || b.contains_human(),
            Expr::Div(box a, box b) => a.contains_human() || b.contains_human(),
        }
    }

    fn force_eval(&self) -> Option<i64> {
        match self {
            Expr::Human => None,
            Expr::Const(i) => Some(*i),
            Expr::Add(box a, box b) => Some(a.force_eval()? + b.force_eval()?),
            Expr::Sub(box a, box b) => Some(a.force_eval()? - b.force_eval()?),
            Expr::Mul(box a, box b) => Some(a.force_eval()? * b.force_eval()?),
            Expr::Div(box a, box b) => Some(a.force_eval()? / b.force_eval()?),
        }
    }
}

fn solve(expr: Expr, target: i64) -> i64 {
    match expr {
        Expr::Human => target,
        Expr::Const(_) => unreachable!(),
        Expr::Add(a, b) => match (a.force_eval(), b.force_eval()) {
            (None, Some(i)) => solve(*a, target - i),
            (Some(i), None) => solve(*b, target - i),
            _ => unreachable!(),
        },
        Expr::Sub(a, b) => match (a.force_eval(), b.force_eval()) {
            (None, Some(i)) => solve(*a, target + i),
            (Some(i), None) => solve(*b, i - target),
            _ => unreachable!(),
        },
        Expr::Mul(a, b) => match (a.force_eval(), b.force_eval()) {
            (None, Some(i)) => solve(*a, target / i),
            (Some(i), None) => solve(*b, target / i),
            _ => unreachable!(),
        },
        Expr::Div(a, b) => match (a.force_eval(), b.force_eval()) {
            (None, Some(i)) => solve(*a, target * i),
            (Some(i), None) => solve(*b, i / target),
            _ => unreachable!(),
        },
    }
}
