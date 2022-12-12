use advent_of_code::helpers::{AocResult, Folder};
use std::{collections::HashSet, str::FromStr};

const DAY: u8 = 9;
type Input<'a> = &'a [Line];
type Solution = Option<usize>;

pub struct Line {
    direction: Direction,
    movements: u8,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        let mut splits = s.split_ascii_whitespace();
        Ok(Self {
            direction: splits.next().ok_or_else(|| "Empty line".to_string())?.try_into()?,
            movements: splits
                .next()
                .ok_or_else(|| "Line missing movements".to_string())?
                .parse()
                .map_err(|_| format!("Invalid number of movements on line {s}"))?,
        })
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> AocResult<Self> {
        match value.as_bytes()[0] {
            b'U' => Ok(Direction::Up),
            b'R' => Ok(Direction::Right),
            b'D' => Ok(Direction::Down),
            b'L' => Ok(Direction::Left),
            _ => Err(format!("Unknown direction: {value}")),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position(i32, i32); // (horizontal, vertical)

impl Position {
    fn new() -> Self {
        Self(0, 0)
    }

    fn move_one(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.1 += 1,
            Direction::Right => self.0 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
        }
    }

    fn adjust_to(&mut self, leader: Self) {
        let horizontal_gap = leader.0 - self.0;
        let vertical_gap = leader.1 - self.1;

        if matches!(horizontal_gap, -1..=1) && matches!(vertical_gap, -1..=1) {
            return;
        }

        match horizontal_gap {
            0 => {}
            1.. => self.move_one(&Direction::Right),
            _ => self.move_one(&Direction::Left),
        }

        match vertical_gap {
            0 => {}
            1 | 2 => self.move_one(&Direction::Up),
            -1 | -2 => self.move_one(&Direction::Down),
            _ => unreachable!("the maximum distance is two steps in any direction"),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

fn run<const LENGTH: usize>(input: Input, mut rope: [Position; LENGTH]) -> Solution {
	let mut visited = HashSet::new();
    visited.insert(rope[LENGTH - 1]);

    for line in input {
        for _ in 0..line.movements {
			rope[0].move_one(&line.direction);
			for n in 1..LENGTH {
				rope[n].adjust_to(rope[n - 1]);
			}
			visited.insert(rope[LENGTH - 1]);
        }
    }
    Some(visited.len())
}

pub fn part_one(input: Input) -> Solution {
	let rope: [Position; 2] = Default::default();
    run(input, rope)
}

pub fn part_two(input: Input) -> Solution {
	let rope: [Position; 10] = Default::default();
    run(input, rope)
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
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_two(&input), Some(1));
    }

	#[test]
	fn test_part_two_example_two() {
		let input = vec![
			Line::from_str("R 5").unwrap(),
			Line::from_str("U 8").unwrap(),
			Line::from_str("L 8").unwrap(),
			Line::from_str("D 3").unwrap(),
			Line::from_str("R 17").unwrap(),
			Line::from_str("D 10").unwrap(),
			Line::from_str("L 25").unwrap(),
			Line::from_str("U 20").unwrap(),
		];
		assert_eq!(part_two(&input), Some(36));
	}
}
