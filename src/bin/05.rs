use std::{
    collections::VecDeque,
    fmt::Debug,
    str::{FromStr, SplitWhitespace},
};

use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 5;
type Input<'a> = &'a [Line];
type Solution = Option<String>;

pub enum Line {
    Block(String),
    Index,
    Empty,
    Move(Movement),
}

impl FromStr for Line {
	type Err = String;

    fn from_str(input: &str) -> AocResult<Self> {
        match input.trim_start().as_bytes().first() {
            None => Ok(Self::Empty),
            Some(b'[') => Ok(Self::Block(input.to_owned())),
            Some(b'1') => Ok(Self::Index),
            Some(b'm') => {
                let mut splits = input.split_whitespace();
                let quantity = next_number(&mut splits);
                let from = next_number(&mut splits);
                let to = next_number(&mut splits);
                Ok(Self::Move(Movement { quantity, from, to }))
            }
            _ => Err(format!("Unexpected line: '{input}'")),
        }
    }
}

struct Stacks {
    vec: Vec<VecDeque<u8>>,
}

impl Stacks {
    fn new() -> Stacks {
        Stacks { vec: Vec::new() }
    }

    /// Adds the cargo to the desired crate, if it doesn't exists yet, creates it
    fn push_to(&mut self, index: usize, cargo: u8) {
        while index >= self.vec.len() {
            self.vec.push(VecDeque::new())
        }
        self.vec[index].push_back(cargo)
    }

	/// Moves elements from one stack to another one by one
    fn move_cargo_single(&mut self, movement: &Movement) {
        for _ in 0..movement.quantity {
            let cargo = self.vec[movement.from - 1].pop_front().expect("content in a stack to be moved");
            self.vec[movement.to - 1].push_front(cargo)
        }
    }

	/// Moves elements from one stack to other by chunks
	fn move_cargo_block(&mut self, movement: &Movement) {
		let elements: Vec<u8> = self.vec[movement.from - 1].drain(0..movement.quantity as usize).rev().collect();
		for element in elements {
			self.vec[movement.to - 1].push_front(element);
		}
	}

    /// Returns a string with the IDs of the cargos on top of each stack
    fn print_top(&self) -> String {
        let bytes: Vec<u8> = self.vec.iter().map(|v| *v.get(0).unwrap()).collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

pub struct Movement {
    quantity: u32,
    from: usize,
    to: usize,
}

fn next_number<T>(splits: &mut SplitWhitespace) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    _ = splits.next(); // word
    str::parse(splits.next().expect("movement lines to have even elements")).expect("even elements to be numbers")
}

fn get_id(chunk: &[u8]) -> Option<&u8> {
    match chunk.get(1) {
        Some(id) if !id.is_ascii_whitespace() => Some(id),
        _ => None,
    }
}

fn process_cargo_line(line: &str, stacks: &mut Stacks) {
    line.as_bytes()
        .chunks(4)
        .map(get_id)
        .enumerate()
        .for_each(|(i, chunk)| {
            if let Some(id) = chunk {
                stacks.push_to(i, *id);
            }
        });
}

fn process_instructions(input: Input, action: impl Fn(&mut Stacks, &Movement)) -> String {
	let mut stacks = Stacks::new();

    input
        .iter()
        .for_each(|line| match line {
            Line::Block(line) => process_cargo_line(line, &mut stacks),
            Line::Move(movement) => action(&mut stacks, movement),
            _ => {}
        });

    stacks.print_top()
}

pub fn part_one(input: Input) -> Solution {
    Some(process_instructions(input, Stacks::move_cargo_single))
}

pub fn part_two(input: Input) -> Solution {
    Some(process_instructions(input, Stacks::move_cargo_block))
}

fn main() -> AocResult<()> {
	let setup = || {
        advent_of_code::helpers::read_input(Folder::Inputs, DAY)
    };

    let input = advent_of_code::load!(setup)?;
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
		let input = &advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
		let input = &advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
