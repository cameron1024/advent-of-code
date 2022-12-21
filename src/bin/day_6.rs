const INPUT: &str = include_str!("../inputs/day_6");

fn main() {
    let result = part_1();
    println!("{result}");
    let result = part_2();
    println!("{result}");
}

fn solve(input: &str, distinct_len: usize) -> usize {
    input
        .as_bytes()
        .windows(distinct_len)
        .enumerate()
        .find_map(
            |(index, window)| {
                if all_ne(window) {
                    Some(index)
                } else {
                    None
                }
            },
        )
        .unwrap()
        + distinct_len
}

fn part_1() -> usize {
    solve(INPUT, 4)
}

fn part_2() -> usize {
    solve(INPUT, 14)
}

fn all_ne<T: PartialEq>(elems: &[T]) -> bool {
    let len = elems.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if elems[i] == elems[j] {
                return false;
            }
        }
    }

    true
}

#[test]
fn example() {
    let x = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(solve(x, 14), 19);
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
}
