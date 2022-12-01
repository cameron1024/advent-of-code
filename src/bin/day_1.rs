const INPUT: &str = include_str!("../inputs/day_1");

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut max = 0;
    let mut current_elf = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            if current_elf > max {
                max = current_elf;
            }
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    if current_elf > max {
        max = current_elf;
    }

    println!("{max}");
}

fn part_2() {
    let mut elves = vec![];
    let mut current_elf = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>().unwrap();
        }
    }

    elves.push(current_elf);
    elves.sort_unstable();
    elves.reverse();
    let sum: i32 = elves.into_iter().take(3).sum();
    println!("{sum}");
}
