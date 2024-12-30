use std::fs::read_to_string;

fn main() -> Result<(), anyhow::Error> {
    let input = read_to_string("./day_02/input.txt")?;
    println!("{}", part_1(&input)?);
    println!("{}", part_2(&input)?);

    Ok(())
}

pub fn parse_input(file: &str) -> Vec<Vec<usize>> {
    let lines = file.lines();
    let mut lists = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let list = line
            .split(" ")
            .map(|s| s.parse::<usize>())
            .flat_map(Result::ok)
            .collect::<Vec<usize>>();

        lists.push(list);
    }

    lists
}

pub fn part_1(input: &str) -> Result<usize, anyhow::Error> {
    let lists = parse_input(input);
    let number_of_safe_lists = lists.iter().fold(
        0,
        |acc, curr| if day_02::list_is_safe(curr) { acc + 1 } else { acc },
    );

    Ok(number_of_safe_lists)
}

pub fn part_2(input: &str) -> Result<usize, anyhow::Error> {
    let lists = parse_input(input);
    let mut number_of_safe_lists = 0;

    for list in lists.iter() {
        if day_02::list_is_safe(list) {
            number_of_safe_lists += 1;
            continue;
        }

        let reduced_lists = (0..list.len())
            .map(|i| {
                let mut reduced_list = list.clone();
                reduced_list.remove(i);

                reduced_list
            })
            .collect::<Vec<Vec<usize>>>();

        if reduced_lists.iter().any(day_02::list_is_safe) {
            number_of_safe_lists += 1;
        }
    }

    Ok(number_of_safe_lists)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_1_works() -> Result<(), anyhow::Error> {
        assert_eq!(part_1(INPUT)?, 2);

        Ok(())
    }

    #[test]
    fn part_2_works() -> Result<(), anyhow::Error> {
        assert_eq!(part_2(INPUT)?, 4);

        Ok(())
    }
}
