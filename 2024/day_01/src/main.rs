use std::fs::read_to_string;

fn main() -> Result<(), anyhow::Error> {
    let input = read_to_string("./day_01/input.txt")?;
    println!("{}", part_1(&input)?);
    println!("{}", part_2(&input)?);

    Ok(())
}

pub fn parse_input(file: &str) -> Result<(Vec<usize>, Vec<usize>), anyhow::Error> {
    let lines = file.lines();
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if let Some((left, right)) = line.split_once("   ") {
            left_list.push(left.parse::<usize>()?);
            right_list.push(right.parse::<usize>()?);
        }
    }

    Ok((left_list, right_list))
}

pub fn part_1(input: &str) -> Result<usize, anyhow::Error> {
    let (mut left_list, mut right_list) = parse_input(input)?;
    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        if left > right {
            sum += left - right;
        } else {
            sum += right - left;
        }
    }

    Ok(sum)
}

pub fn part_2(input: &str) -> Result<usize, anyhow::Error> {
    let (left_list, right_list) = parse_input(input)?;
    let mut total = 0;

    for i in left_list.iter() {
        for j in right_list.iter() {
            if i == j {
                total += i;
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_1_works() -> Result<(), anyhow::Error> {
        assert_eq!(part_1(INPUT)?, 11);

        Ok(())
    }

    #[test]
    fn part_2_works() -> Result<(), anyhow::Error> {
        assert_eq!(part_2(INPUT)?, 31);

        Ok(())
    }
}
