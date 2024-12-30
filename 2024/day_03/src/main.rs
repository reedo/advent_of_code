use day_03::extract_mul_fns;
use std::fs::read_to_string;

fn main() -> Result<(), anyhow::Error> {
    let input = read_to_string("./day_03/input.txt")?;
    println!("{}", part_1(&input)?);
    println!("{}", part_2(&input)?);

    Ok(())
}

pub fn part_1(input: &str) -> Result<usize, anyhow::Error> {
    let mut total = 0;
    for line in input.lines() {
        let mul_fns = extract_mul_fns(line)?;
        for mul_fn in mul_fns.iter() {
            total += mul_fn.x * mul_fn.y;
        }
    }

    Ok(total)
}

pub fn part_2(_input: &str) -> Result<usize, anyhow::Error> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_03::*;

    const INPUT: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part_1_works() -> Result<(), anyhow::Error> {
        assert_eq!(
            extract_mul_fns(INPUT)?,
            vec![
                MulFn { x: 2, y: 4 },
                MulFn { x: 5, y: 5 },
                MulFn { x: 11, y: 8 },
                MulFn { x: 8, y: 5 }
            ]
        );

        assert_eq!(part_1(INPUT)?, 161);

        Ok(())
    }

    #[test]
    fn part_2_works() -> Result<(), anyhow::Error> {
        assert_eq!(part_2(INPUT)?, 48);

        Ok(())
    }
}
