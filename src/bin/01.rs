use advent_of_code::helpers::{AocResult, Folder};
use std::{collections::BinaryHeap, str::FromStr};

const DAY: u8 = 1;
type Input<'a> = &'a [Line];
type Solution = Option<u32>;

pub enum Line {
    Empty,
    Calories(u32),
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        if s.is_empty() {
            Ok(Self::Empty)
        } else {
            let calories: u32 =
                FromStr::from_str(s).map_err(|_| format!("invalid calories number: {s}"))?;
            Ok(Self::Calories(calories))
        }
    }
}

fn read_lines(input: Input, mut if_empty: impl FnMut(u32)) {
    let mut calories_count = 0_u32;

    input.iter().for_each(|line| match line {
        Line::Empty => {
            if_empty(calories_count);
            calories_count = 0;
        }
        Line::Calories(calories) => calories_count += calories,
    });
    if_empty(calories_count);
}

pub fn part_one(input: Input) -> Solution {
    let mut top_calories = 0;

    // simple solution with count and max comparison, memory efficient
    read_lines(input, |calories_count| {
        if calories_count > top_calories {
            top_calories = calories_count;
        }
    });

    Some(top_calories)
}

pub fn part_two(input: Input) -> Solution {
    let mut top_calories = BinaryHeap::with_capacity(3);

    // let's just use heaps, it's actually faster tho
    read_lines(input, |calories_count| {
        top_calories.push(calories_count);
    });

    // let sum = top_calories.into_iter_sorted().take(3).sum() is nightly :(
    let sum = (0..3)
        .map(|_| top_calories.pop().expect("heap to have at least 3 items"))
        .into_iter()
        .sum();
    Some(sum)
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
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_two(&input), Some(45000));
    }
}
