use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day_15/input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    0
}

fn part_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"";

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}
