use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day_03/input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .map(|pair| priority_of(&get_shared_char(*pair)))
        .sum()
}

fn part_2(input: &str) -> u32 {
    0
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .collect::<Vec<(&str, &str)>>()
}

fn get_shared_char(pair: (&str, &str)) -> char {
    for c in pair.0.chars() {
        if pair.1.contains(c) {
            return c;
        }
    }

    '_'
}

fn priority_of(c: &char) -> u32 {
    match c {
        'a'..='z' => (*c) as u32 - 96,
        'A'..='Z' => (*c) as u32 - 38,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_is_calculated_correctly() {
        assert_eq!(priority_of(&'a'), 1);
        assert_eq!(priority_of(&'z'), 26);
        assert_eq!(priority_of(&'A'), 27);
        assert_eq!(priority_of(&'Z'), 52);
    }

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 157);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}
