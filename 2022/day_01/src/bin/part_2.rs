use day_01::get_parsed_input;

fn main() -> Result<(), std::io::Error> {
    let elves = get_parsed_input()?;
    let mut sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    sums.sort();

    let mut top_three_sum = sums.pop().unwrap_or(0);
    top_three_sum += sums.pop().unwrap_or(0);
    top_three_sum += sums.pop().unwrap_or(0);

    println!("Top 3 = {:?}", top_three_sum);

    Ok(())
}
