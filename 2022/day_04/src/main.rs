mod cleaning_range;

use std::fs::read_to_string;

use cleaning_range::CleaningRange;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|pair| pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0))
        .count()
}

fn part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|pair| pair.0.overlaps(&pair.1))
        .count()
}

fn parse_input(input: &str) -> Vec<(CleaningRange, CleaningRange)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap_or(("", ""));
            (x.parse().unwrap_or_default(), y.parse().unwrap_or_default())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 4);
    }
}
