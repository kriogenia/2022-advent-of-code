use std::str::FromStr;

use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 4;
type Input<'a> = &'a [Line];
type Solution = Option<u32>;

const RANGE_SEPARATOR: u8 = b',';
const SECTION_SEPARATOR: u8 = b'-';

pub struct Line(SectionRange, SectionRange);

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        let split_point = find_separator(s, RANGE_SEPARATOR)?;
        let left = SectionRange::from_str(&s[..split_point])?;
        let right = SectionRange::from_str(&s[split_point + 1..])?;
        Ok(Self(left, right))
    }
}

struct SectionRange {
    from: i32,
    to: i32,
}

impl FromStr for SectionRange {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        let split_point = find_separator(s, SECTION_SEPARATOR)?;
        let from = str::parse(&s[..split_point])
            .map_err(|_| format!("Wrong format on section: {s}. Invalid number"))?;
        let to = str::parse(&s[split_point + 1..])
            .map_err(|_| format!("Wrong format on section: {s}. Invalid number"))?;
        Ok(SectionRange { from, to })
    }
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        self.to >= other.from && self.from <= other.to
    }
}

fn find_separator(s: &str, separator: u8) -> AocResult<usize> {
    s.bytes()
        .position(|c| c == separator)
        .ok_or(format!("Wrong format on: '{s}'. Missing '-'."))
}

fn count_ranges_matching(input: Input, filter: impl FnMut(&&Line) -> bool) -> u32 {
    (*input).iter().filter(filter).count() as u32
}

pub fn part_one(input: Input) -> Solution {
    Some(count_ranges_matching(input, |Line(l, r)| {
        l.contains(r) || r.contains(l)
    }))
}

pub fn part_two(input: Input) -> Solution {
    Some(count_ranges_matching(input, |Line(l, r)| l.overlaps(r)))
}

fn main() -> AocResult<()> {
    let input = advent_of_code::helpers::read_input(Folder::Inputs, DAY)?;
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_two(&input), Some(4));
    }
}
