use std::{collections::HashMap};

const LOWERCASE_SUBSTRACTION: u8 = b'a' - 1;
const UPPERCASE_SUBSTRACTION: u8 = b'A' - 27;

fn get_priority(value: u8) -> u32 {
    if value < b'a' {
        value - UPPERCASE_SUBSTRACTION
    } else {
        value - LOWERCASE_SUBSTRACTION
    }
    .into()
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

fn find_mistaken(line: &str) -> Option<u32> {
	let mut map = HashMap::with_capacity(line.len());
	let (l, r) = line.as_bytes().split_at(line.len() / 2);
	for (left, right) in l.into_iter().zip(r) {
		if let Some(Side::Right) = map.insert(left, Side::Left) {
			return Some(get_priority(*left));
		}
		if let Some(Side::Left) = map.insert(right, Side::Right) {
			return Some(get_priority(*right));
		}
	}
	return None;
}

pub fn part_one(input: &str) -> Option<u32> {
	let mut sum = 0;
	for line in input.lines() {
		sum += find_mistaken(&line).expect("all lines should have a mistaken item");
	}
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
