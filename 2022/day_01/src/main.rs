use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

pub fn parse_input(file: &str) -> Result<Vec<Vec<u32>>, std::io::Error> {
    let lines = file.lines();

    let mut elves: Vec<_> = Vec::new();
    elves.push(Vec::new());

    // Parse the data.
    for line in lines {
        if line.is_empty() {
            // If the line is blank, start a new `Elf` and continue.
            elves.push(Vec::new());
        } else {
            // If the line is not blank, push the value to `snacks` in the
            // last `Elf` in the `elves` vector.
            elves
                .last_mut()
                .expect("Could not get the last Elf")
                .push(line.parse::<u32>().expect("Could not parse into an int"));
        }
    }

    Ok(elves)
}

pub fn part_1(input: &str) -> u32 {
    let elves = parse_input(input).expect("Failed to get parsed input");

    let sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();

    *sums.iter().max().unwrap_or(&0)
}

pub fn part_2(input: &str) -> u32 {
    let elves = parse_input(input).expect("Failed to get parsed input");

    let mut sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    sums.sort();

    let mut top_three_sum = sums.pop().unwrap_or(0);
    top_three_sum += sums.pop().unwrap_or(0);
    top_three_sum += sums.pop().unwrap_or(0);

    top_three_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), 24_000)
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), 45_000)
    }
}
