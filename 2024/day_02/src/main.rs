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

fn all_numbers_are_close(list: &[usize]) -> bool {
    let mut prev = list[0];
    for curr in list[1..].iter() {
        let diff = curr.abs_diff(prev);
        if (1..=3).contains(&diff) {
            prev = *curr;
            continue;
        } else {
            return false;
        }
    }

    true
}

fn list_is_safe(list: &[usize]) -> bool {
    let is_ascending = list.is_sorted_by(PartialOrd::gt);
    let is_descending = list.is_sorted_by(PartialOrd::lt);

    (is_ascending || is_descending) && all_numbers_are_close(list)
}

pub fn part_1(input: &str) -> Result<usize, anyhow::Error> {
    let lists = parse_input(input);
    let number_of_safe_lists = lists.iter().fold(
        0,
        |acc, curr| if list_is_safe(curr) { acc + 1 } else { acc },
    );

    Ok(number_of_safe_lists)
}

pub fn part_2(input: &str) -> Result<usize, anyhow::Error> {
    let lists = parse_input(input);
    let mut number_of_safe_lists = 0;

    for list in lists.iter() {
        if list_is_safe(list) {
            number_of_safe_lists += 1;
            continue;
        }

        let reduced_list = (0..list.len())
            .map(|i| {
                let mut reduced_list = list.clone();
                reduced_list.remove(i);

                reduced_list
            })
            .collect::<Vec<Vec<usize>>>();

        if reduced_list.iter().any(|list| list_is_safe(list)) {
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
