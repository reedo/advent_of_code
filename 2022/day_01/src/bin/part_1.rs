use day_01::get_parsed_input;

fn main() -> Result<(), std::io::Error> {
    let elves = get_parsed_input()?;
    let sums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    println!("Max = {}", sums.iter().max().unwrap_or(&0));

    Ok(())
}
