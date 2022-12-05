use std::{
    collections::VecDeque,
    fmt::Debug,
    str::{FromStr, SplitWhitespace},
};

struct Stacks {
    vec: Vec<VecDeque<u8>>,
}

impl Stacks {
    fn new() -> Stacks {
        Stacks { vec: Vec::new() }
    }

    fn expand(&mut self) {
        self.vec.push(VecDeque::new())
    }

    /// Adds the cargo to the desired crate, if it doesn't exists yet, creates it
    fn push_to(&mut self, index: usize, cargo: u8) {
        while index >= self.vec.len() {
            self.expand()
        }
        self.vec[index].push_back(cargo)
    }

    fn move_cargo(&mut self, movement: Movement) {
        for _ in 0..movement.quantity {
            let cargo = self.vec[movement.from - 1].pop_front().expect("content in a stack to be moved");
            self.vec[movement.to - 1].push_front(cargo)
        }
    }

    /// Returns a string with the IDs of the cargos on top of each stack
    fn print_top(&self) -> String {
        let bytes: Vec<u8> = self.vec.iter().map(|v| *v.get(0).unwrap()).collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

struct Movement {
    quantity: u32,
    from: usize,
    to: usize,
}

enum Line<'a> {
    Block(&'a str),
    Index,
    Empty,
    Move(Movement),
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(input: &'a str) -> Self {
        match input.trim_start().as_bytes().first() {
            None => Self::Empty,
            Some(b'[') => Self::Block(input),
            Some(b'1') => Self::Index,
            Some(b'm') => {
                let mut splits = input.split_whitespace();
                let quantity = next_number(&mut splits);
                let from = next_number(&mut splits);
                let to = next_number(&mut splits);
                Self::Move(Movement { quantity, from, to })
            }
            _ => unreachable!("unexpected line"),
        }
    }
}

fn next_number<T>(splits: &mut SplitWhitespace) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    _ = splits.next(); // word
    str::parse(splits.next().expect("movement lines to have even elements")).expect("even elements to be numbers")
}

fn read_chunk(chunk: &[u8]) -> Option<&u8> {
    match chunk.get(1) {
        Some(id) if !id.is_ascii_whitespace() => Some(id),
        _ => None,
    }
}

fn process_cargo_line(line: &str, stacks: &mut Stacks) {
    line.as_bytes()
        .chunks(4)
        .map(read_chunk)
        .enumerate()
        .for_each(|(i, chunk)| {
            if let Some(id) = chunk {
                stacks.push_to(i, *id);
            }
        });
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = Stacks::new();

    input
        .lines()
        .map(Line::from)
        .for_each(|line| match line {
            Line::Block(line) => process_cargo_line(line, &mut stacks),
            Line::Move(movement) => stacks.move_cargo(movement),
            _ => {}
        });

    Some(stacks.print_top())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
