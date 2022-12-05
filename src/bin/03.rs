use std::{collections::HashMap};

use advent_of_code::helpers::Folder;

const DAY: u8 = 3;
type Input<'a> = &'a str;
type Solution = Option<u32>;

const ELVES_PER_GROUP: usize = 3;
const MAX_LINE_LENGTH: usize = 50;
const LOWERCASE_SUBSTRACTION: u8 = b'a' - 1;
const UPPERCASE_SUBSTRACTION: u8 = b'A' - 27;

enum Side {
    Left,
    Right,
}

fn get_priority(value: u8) -> u32 {
    if value < b'a' {
        value - UPPERCASE_SUBSTRACTION
    } else {
        value - LOWERCASE_SUBSTRACTION
    }
    .into()
}

fn find_mistaken(line: &str) -> Option<u32> {
	let mut map = HashMap::with_capacity(MAX_LINE_LENGTH);
	let (l, r) = line.as_bytes().split_at(line.len() / 2);
	for (left, right) in l.iter().zip(r) {
		if let Some(Side::Right) = map.insert(left, Side::Left) {
			return Some(get_priority(*left));
		}
		if let Some(Side::Left) = map.insert(right, Side::Right) {
			return Some(get_priority(*right));
		}
	}
	None
}

fn find_common(group: &[&str]) -> Option<u32> {
	let mut map = HashMap::with_capacity(MAX_LINE_LENGTH);
	// as we only care if the char is present in all the group we can use the first elf as the initial cache
	for b in group[0].as_bytes() {
		map.insert(*b, 1);
	}
	// then for the second we only care to specify if they are also present
	for b in group[1].as_bytes() {
		map.entry(*b).and_modify(|v| *v = 2);
	}
	// and the last should pick the first item that is already present in the other two
	for b in group[2].as_bytes() {
		if let Some(&2) = map.get(b) {
			return Some(get_priority(*b));
		}
	}
	None
}

pub fn part_one(input: Input) -> Solution {
	let mut sum = 0;
	for line in input.lines() {
		sum += find_mistaken(line).expect("rucksacks to have at least one wrong item");
	}
    Some(sum)
}

pub fn part_two(input: Input) -> Solution {
	let mut sum = 0;
	for group in input.lines().collect::<Vec<&str>>().chunks(ELVES_PER_GROUP) {
		sum += find_common(group).expect("groups to have a common item")
	}
	Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file(Folder::Inputs, DAY);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        assert_eq!(part_two(&input), Some(70));
    }
}
