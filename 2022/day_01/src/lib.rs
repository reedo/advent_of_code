use std::fs::read_to_string;

pub fn get_parsed_input() -> Result<Vec<Vec<u32>>, std::io::Error> {
    let file = read_to_string("./day_01/input.txt")?;
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
