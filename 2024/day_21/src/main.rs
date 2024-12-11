use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

pub fn parse_input(file: &str) -> Result<(), std::io::Error> {
    todo!()
}

pub fn part_1(input: &str) -> &str {
    ""
}

pub fn part_2(input: &str) -> &str {
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "")
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "")
    }
}
