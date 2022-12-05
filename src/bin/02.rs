use std::{str::FromStr};

use advent_of_code::helpers::AocResult;

const DAY: u8 = 2;
type Input<'a> = &'a [Line];
type Solution = Option<u32>;

pub struct Line(u8, u8);

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
			[left, _, right, ..] => Ok(Line(*left, *right)),
			_ => Err(format!("line not matching `[0.9] [0-9].*`: {s}"))
		}
    }
}

#[derive(Clone, PartialEq)]
enum Play {
	Rock,
	Paper,
	Scissors
}

impl Play {
	fn point_value(&self) -> u32 {
		match self {
			Play::Rock => 1,
			Play::Paper => 2,
			Play::Scissors => 3
		}
	}

	fn wins_to(&self) -> Self {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper
		}
	}

	// it could be replaced for wins_to().wins_to() to reduce code, but this one is actually faster
	fn loses_to(&self) -> Self {
		match self {
			Self::Rock => Self::Paper,
			Self::Paper => Self::Scissors,
			Self::Scissors => Self::Rock
		}
	}

    fn clash_with(&self, other: &Self) -> RoundResult {
		if self == other {
			RoundResult::Draw
		} else if self.wins_to() == *other {
			RoundResult::Win
		} else {
			RoundResult::Loss
		}
    }
}

impl TryFrom<u8> for Play {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'A' | b'X' => Ok(Self::Rock),
			b'B' | b'Y' => Ok(Self::Paper),
			b'C' | b'Z' => Ok(Self::Scissors),
			_ => Err(())
		}
    }
}

enum RoundResult {
	Win,
	Draw,
	Loss
}

impl RoundResult {
	fn value(&self) -> u32 {
		match self {
			Self::Loss => 0,
			Self::Draw => 3,
			Self::Win => 6
		}
	}
}

impl TryFrom<u8> for RoundResult {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'X' => Ok(RoundResult::Loss),
			b'Y' => Ok(RoundResult::Draw),
			b'Z' => Ok(RoundResult::Win),
			_ => Err(())
		}
    }
}

fn needed_play_for(result: RoundResult, against: &Play) -> Play {
	match result {
		RoundResult::Draw => against.clone(),
		RoundResult::Loss => against.wins_to(),
		RoundResult::Win => against.loses_to()
	}
}

// I'm pretty proud of this one, let's just build what we need with a single function
fn parse_line<L, R>(input: &Line) -> Option<(L, R)> 
where
	L: TryFrom<u8>,
	R: TryFrom<u8>,
{
	match (input.0.try_into(), input.1.try_into()) {
		(Ok(left), Ok(right)) => Some((left, right)),
		_ =>  None
	}
}

fn process_plays(input: Input, calculate_play: impl Fn(&Line) -> (Play, Play)) -> u32 {
	let mut total_score = 0;
	for line in input.iter() {
		let (my_play, opponent_play) = calculate_play(line);
		total_score += my_play.point_value() + my_play.clash_with(&opponent_play).value();
	}
	total_score
}

pub fn part_one(input: Input) -> Solution {
	let score = process_plays(input, |line| {
		parse_line(line).expect("line with two correct plays")
	});
	Some(score)
}

pub fn part_two(input: Input) -> Solution {
	let score = process_plays(input, |line| {
		let (opponent_play, intended_result) = parse_line(line).expect("line with play and result");
		let to_play = needed_play_for(intended_result, &opponent_play);
		(to_play, opponent_play)
	});
	Some(score)
}

fn main() -> AocResult<()> {
    let input = advent_of_code::helpers::read_input("inputs", DAY)?;
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::helpers::read_input("examples", DAY).unwrap();
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::helpers::read_input("examples", DAY).unwrap();
        assert_eq!(part_two(&input), Some(12));
    }
}
