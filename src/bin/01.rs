use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
	let mut calories_counter = 0_u32;
	let mut top_calories = 0;

	for line in input.lines() {
		if line.is_empty() {
			if calories_counter > top_calories {
				top_calories = calories_counter;
			}
			calories_counter = 0;
			continue;
		}
		let calories: u32 = FromStr::from_str(line).expect("the text to be a 32bit integer");
		calories_counter += calories;
	}
	return Some(top_calories);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
