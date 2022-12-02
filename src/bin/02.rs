use std::cmp::Ordering;

#[derive(PartialEq)]
enum Play {
	Rock,
	Paper,
	Scissors
}

impl Play {
	fn get_points(&self) -> u32 {
		match self {
			Play::Rock => 1,
			Play::Paper => 2,
			Play::Scissors => 3
		}
	}
}

impl TryFrom<Option<&str>> for Play {
    type Error = ();

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
		match value.unwrap_or("Err") {
			"A" | "X" => Ok(Play::Rock),
			"B" | "Y" => Ok(Play::Paper),
			"C" | "Z" => Ok(Play::Scissors),
			_ => Err(())
		}
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		use Play::*;
		Some(match (self, other) {
			(Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Ordering::Greater,
			(Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Ordering::Less,
			_ => Ordering::Equal
		})
    }
}

fn parse_play(input: &str) -> Option<(Play, Play)> {
	let mut plays = input.split_whitespace();
	match (plays.next().try_into(), plays.next().try_into()) {
		(Ok(left), Ok(right)) => Some((left, right)),
		_ =>  None
	}
}

fn calculate_score(my_play: Play, opponent_play: Play) -> u32 {
	let victory_points = match my_play.partial_cmp(&opponent_play).unwrap() {
		Ordering::Less => 0,
		Ordering::Equal => 3,
		Ordering::Greater => 6
	};
	my_play.get_points() + victory_points
}

pub fn part_one(input: &str) -> Option<u32> {
	let mut total_score = 0;
    for line in input.lines() {
		let (opponent_play, my_play) = parse_play(line).expect("line with two correct plays");
		total_score += calculate_score(my_play, opponent_play);
	}
	Some(total_score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
