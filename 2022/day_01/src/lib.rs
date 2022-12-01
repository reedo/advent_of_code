use std::fs::read_to_string;

pub fn get_parsed_input() -> Result<Vec<Vec<u32>>, std::io::Error> {
    let file = read_to_string("./input.txt")?;
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

pub fn part_1() -> u32 {
    let elves = get_parsed_input().expect("Failed to get parsed input");

    let sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();

    *sums.iter().max().unwrap_or(&0)
}

pub fn part_2() -> u32 {
    let elves = get_parsed_input().expect("Failed to get parsed input");

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

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(), 69501)
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(), 202346)
    }
}
