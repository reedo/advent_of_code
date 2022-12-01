// --- Day 1: Calorie Counting ---
// https://adventofcode.com/2022/day/1

mod input;

use crate::input::INPUT;

fn main() {
    let lines = INPUT.lines();

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

    // Part 1
    let sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    println!("Max = {}", sums.iter().max().unwrap_or(&0));

    // Part 2
    let mut sorted_sums = sums.clone();
    sorted_sums.sort();
    let mut top_three_sum = sorted_sums.pop().unwrap_or(0);
    top_three_sum += sorted_sums.pop().unwrap_or(0);
    top_three_sum += sorted_sums.pop().unwrap_or(0);

    println!("Top 3 = {:?}", top_three_sum);
}
