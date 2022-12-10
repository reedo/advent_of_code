use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;

    for line in lines {
        match line {
            "A X" => score += 4, // Rock (1) + Draw (3)
            "A Y" => score += 8, // Paper (2) + Win (6)
            "A Z" => score += 3, // Scissors (3) + Lose (0)

            "B X" => score += 1, // Rock (1) + Lose (0)
            "B Y" => score += 5, // Paper (2) + Draw (3)
            "B Z" => score += 9, // Scissors (3) + Win (6)

            "C X" => score += 7, // Rock (1) + Win (6)
            "C Y" => score += 2, // Paper (2) + Lose (0)
            "C Z" => score += 6, // Scissors (3) + Draw (3)
            _ => (),
        }
    }

    score
}

fn part_2(input: &str) -> u32 {
    let lines = input.lines();
    let mut score = 0;

    for line in lines {
        match line {
            "A X" => score += 3, // Lose (0) + Scissors (3)
            "A Y" => score += 4, // Draw (3) + Rock (1)
            "A Z" => score += 8, // Win (6) + Paper (2)

            "B X" => score += 1, // Lose (0) + Rock (1)
            "B Y" => score += 5, // Draw (3) + Paper (2)
            "B Z" => score += 9, // Win (6) + Scissors (3)

            "C X" => score += 2, // Lose (0) + Paper (2)
            "C Y" => score += 6, // Draw (3) + Scissors (3)
            "C Z" => score += 7, // Win (6) + Rock (1)
            _ => (),
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn example_1() {
        assert_eq!(part_1(INPUT), 15);
    }

    #[test]
    fn example_2() {
        assert_eq!(part_2(INPUT), 12);
    }
}
