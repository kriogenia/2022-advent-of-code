use std::{str::FromStr, collections::BinaryHeap};

fn read_lines(input: &str, mut if_empty: impl FnMut(u32)) {
	let mut calories_count = 0_u32;

	for line in input.lines() {
		if line.is_empty() {
			if_empty(calories_count);
			calories_count = 0;
		} else {
			let calories: u32 = FromStr::from_str(line).expect("the text to be a 32bit integer");
			calories_count += calories;
		}
	}
	if_empty(calories_count);
}

pub fn part_one(input: &str) -> Option<u32> {
	let mut top_calories = 0;

	// simple solution with count and max comparison, memory efficient
	read_lines(input, |calories_count| {
		if calories_count > top_calories {
			top_calories = calories_count;
		}
	});

	Some(top_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut top_calories = BinaryHeap::with_capacity(3);

	// let's just use heaps, it's actually faster tho
	read_lines(input, |calories_count| {
		top_calories.push(calories_count);
	});

	println!("{:?}", top_calories);
	// let sum = top_calories.into_iter_sorted().take(3).sum() is nightly :(
	let sum = (0..3).map(|_| top_calories.pop().unwrap()).into_iter().sum();
	Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
