use std::str::FromStr;

#[derive(Default)]
pub struct CleaningRange {
    pub start: u32,
    pub end: u32,
}

pub enum Error {
    NotARangeString,
    ParseError,
}

impl FromStr for CleaningRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start_str, end_str)) = s.split_once('-') {
            let start: u32 = match start_str.parse() {
                Ok(v) => v,
                Err(_) => return Err(Error::ParseError),
            };
            let end: u32 = match end_str.parse() {
                Ok(v) => v,
                Err(_) => return Err(Error::ParseError),
            };

            Ok(CleaningRange { start, end })
        } else {
            Err(Error::NotARangeString)
        }
    }
}

impl CleaningRange {
    pub fn fully_contains(&self, other: &CleaningRange) -> bool {
        (other.start..=other.end).all(|x| (self.start..=self.end).contains(&x))
    }

    pub fn overlaps(&self, other: &CleaningRange) -> bool {
        (other.start..=other.end).any(|x| (self.start..=self.end).contains(&x))
    }
}
