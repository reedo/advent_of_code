use regex::Regex;

pub fn extract_mul_fns(s: &str) -> Result<Vec<MulFn>, anyhow::Error> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut mul_fns = vec![];
    for (_, [f1, f2]) in re.captures_iter(s).map(|caps| caps.extract()) {
        mul_fns.push(MulFn {
            x: f1.parse()?,
            y: f2.parse()?,
        });
    }

    Ok(mul_fns)
}

#[derive(Debug, PartialEq)]
pub struct MulFn {
    pub x: usize,
    pub y: usize,
}
