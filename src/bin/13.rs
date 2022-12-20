use std::{cmp::Ordering, fmt::Debug};

use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 13;
type Input<'a> = &'a [Pair];
type Solution = Option<usize>;

pub struct Pair {
    left: Data,
    right: Data,
}

impl Pair {
    fn is_right_order(&self) -> bool {
		self.left <= self.right
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{:?}", self.left).unwrap();
        writeln!(f, "{:?}", self.right)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Data {
    List(Vec<Data>),
    Integer(u8),
}

impl TryFrom<&str> for Data {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack = vec![Vec::new()];

        let mut number_acc = None;

        for c in value.as_bytes()[1..value.len() - 1].iter() {
            match c {
                b'[' => stack.push(Vec::new()),
                b'0'..=b'9' => {
                    number_acc = Some(match number_acc.take() {
                        None => c - b'0',
                        Some(acc) => acc * 10 + (c - b'0'),
                    })
                }
                b']' => {
                    let current = stack.len() - 1;
                    if let Some(number) = number_acc.take() {
                        stack.get_mut(current).unwrap().push(Self::Integer(number))
                    }
                    let last = stack.pop().unwrap();
                    stack.get_mut(current - 1).unwrap().push(Self::List(last));
                }
                b',' => {
                    if let Some(number) = number_acc.take() {
                        let current = stack.len() - 1;
                        stack.get_mut(current).unwrap().push(Self::Integer(number))
                    }
                }
                _ => return Err(format!("invalid character: {c}")),
            }
        }
        if let Some(number) = number_acc.take() {
            let current = stack.len() - 1;
            stack.get_mut(current).unwrap().push(Self::Integer(number))
        }

        Ok(Self::List(stack.pop().unwrap()))
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(this), Self::Integer(other)) => this.partial_cmp(other),
            (Self::List(this), Self::List(other)) => {
                for i in 0..this.len() {
                    if i >= other.len() {
                        return Some(Ordering::Greater);
                    }
                    match this[i].cmp(&other[i]) {
                        Ordering::Equal => {}
                        order => return Some(order),
                    }
                }
                this.len().partial_cmp(&other.len())
            }
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).partial_cmp(other),
            (Self::List(_), Self::Integer(_)) => self.partial_cmp(&Self::List(vec![other.clone()])),
        }
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(arg0) => write!(f, "{:?}", arg0),
            Self::Integer(arg0) => write!(f, "{}", arg0),
        }
    }
}

fn parse_packets(input: &str) -> AocResult<Vec<Pair>> {
    let mut pairs = Vec::new();
    let mut lines = input.lines();
    while let (Some(left), Some(right), _empty) = (lines.next(), lines.next(), lines.next()) {
        let left = left.try_into()?;
        let right = right.try_into()?;
        pairs.push(Pair { left, right });
    }
    Ok(pairs)
}

pub fn part_one(input: Input) -> Solution {
	Some(
        input
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_right_order())
            .map(|(i, _)| i + 1)
            .sum()
    )
}

pub fn part_two(input: Input) -> Solution {
    None
}

fn main() -> AocResult<()> {
    let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
        parse_packets(&input)
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
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let input = parse_packets(&input).unwrap();
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        //let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        //assert_eq!(part_two(&input), None);
    }

	#[test]
	fn test_list_vs_list() {
		let input = "[[[[6,8],1,8],9,[[3],10,[5,7,2,4],2],8]]\n[[1,2,[]],[[2,[8,10],0,1,[10]]]]\n";
		let input = &parse_packets(&input).unwrap()[0];
        assert!(!input.is_right_order());
	}
}
