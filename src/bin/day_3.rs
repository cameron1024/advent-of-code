const INPUT: &str = include_str!("../inputs/day_3");

fn main() {
    let result = part_1();
    println!("{result}");
    let result = part_2();
    println!("{result}");
}

fn part_1() -> u32 {
    INPUT
        .lines()
        .map(split)
        .map(|(a, b)| common_char(a, b))
        .map(priority)
        .sum()
}

fn part_2() -> u32 {
    INPUT
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let [s1, s2, s3] = lines else { unreachable!() };
            common_char_3(s1, s2, s3)
        })
        .map(priority)
        .sum()
}

fn split(s: &str) -> (&str, &str) {
    let len = s.len();
    let half_len = len / 2;

    (&s[0..half_len], &s[half_len..len])
}

fn common_char(s1: &str, s2: &str) -> char {
    for c in s1.chars() {
        if s2.contains(c) {
            return c;
        }
    }

    unreachable!()
}

fn common_char_3(s1: &str, s2: &str, s3: &str) -> char {
    for c in s1.chars() {
        if s2.contains(c) && s3.contains(c) {
            return c;
        }
    }

    unreachable!()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(split("asdf"), ("as", "df"));
    }

    #[test]
    fn test_common_char() {
        assert_eq!(common_char("asdf", "fqwer"), 'f');
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
