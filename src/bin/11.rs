use std::{str::FromStr, collections::{HashMap, VecDeque}};

use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 11;
type Input<'a> = &'a mut [Monkey];
type Solution = Option<u32>;

const STARTING_ITEMS_LIST_POINTER: usize = 18;
const OP_DECLARATION_POINTER: usize = 19;
const TEST_NUMBER_POINTER: usize = 21;
const IF_TRUE_INDEX_POINTER: usize = 29;
const IF_FALSE_INDEX_POINTER: usize = 30;

const ROUNDS: usize = 20;

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u32>,
    op: Operation,
    test: u32,
    destiny_if_true: usize,
    destiny_if_false: usize,
	play_count: u32,
}

impl Monkey {
	fn pop(&mut self) -> Option<u32> {
		self.items.pop_front()
	}

	fn push(&mut self, item: u32) {
		self.items.push_back(item);
	}
}

#[derive(Debug)]
enum Operation {
    Sum(Value, Value),
    Mul(Value, Value),
}

impl Operation {
    fn run(&self, value: u32) -> u32 {
        match self {
            Self::Sum(left, right) => left.get(value) + right.get(value),
            Self::Mul(left, right) => left.get(value) * right.get(value),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> AocResult<Self> {
        let value = value.split_at(OP_DECLARATION_POINTER).1;
        let mut parts = value.split_ascii_whitespace();
        let left = Value::try_from(parts.next())
            .map_err(|e| format!("Invalid operation. Left operand {e}: {value}"))?;
        let op = parts.next().ok_or(format!("Missing operand: {value}"))?;
        let right = Value::try_from(parts.next())
            .map_err(|e| format!("Invalid operation. Right operand {e}: {value}"))?;
        match op {
            "+" => Ok(Self::Sum(left, right)),
            "*" => Ok(Self::Mul(left, right)),
            _ => Err(format!("unknown operation: {value}")),
        }
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Literal(u32),
}

impl Value {
    fn get(&self, or: u32) -> u32 {
        match self {
            Self::Old => or,
            Self::Literal(value) => *value,
        }
    }
}

impl TryFrom<Option<&str>> for Value {
    type Error = String;

    fn try_from(value: Option<&str>) -> AocResult<Self> {
        match value {
            Some("old") => Ok(Value::Old),
            Some(number) => Ok(Value::Literal(
                number.parse().map_err(|_| format!("invalid: {number}"))?,
            )),
            None => return Err(format!("missing")),
        }
    }
}

fn parse_monkeys(input: &String) -> AocResult<Vec<Monkey>> {
    let mut monkeys = Vec::new();
    let mut iter = input.lines();
    loop {
        if let None = iter.next() {
            break; // monkeys are declared sequentially so we don't care about their hread
        }
        let items = parse_items(
            iter.next()
                .ok_or("Missing starting items line".to_string())?,
        )?;
        let op = iter
            .next()
            .ok_or("Missing operation line".to_string())?
            .try_into()?;
        let test = parse_number(iter.next(), TEST_NUMBER_POINTER)
            .map_err(|e| format!("Invalid test line: {e}"))?;
        let destiny_if_true = parse_number(iter.next(), IF_TRUE_INDEX_POINTER)
            .map_err(|e| format!("Invalid condition true line: {e}"))?;
        let destiny_if_false = parse_number(iter.next(), IF_FALSE_INDEX_POINTER)
            .map_err(|e| format!("Invalid condition false line: {e}"))?;
        monkeys.push(Monkey {
            items,
            op,
            test,
            destiny_if_true,
            destiny_if_false,
			play_count: 0,
        });
        iter.next(); // jump empty line
    }
    Ok(monkeys)
}

fn parse_items(line: &str) -> AocResult<VecDeque<u32>> {
    let mut items = VecDeque::new();
    let mut numbers = line.split_at(STARTING_ITEMS_LIST_POINTER).1.split(", ");
    while let Some(value) = numbers.next() {
        items.push_back(
            value
                .parse()
                .map_err(|_| format!("invalid item: {value}"))?,
        )
    }
    Ok(items)
}

fn parse_number<T: FromStr>(line: Option<&str>, split: usize) -> AocResult<T> {
    line.map(|s| s.split_at(split).1)
        .ok_or("missing".to_string())?
        .parse()
        .map_err(|_| "failed parsing".to_string())
}

pub fn part_one(monkeys: Input) -> Solution {
	let mut sent = HashMap::new();
	for i in 0..monkeys.len() {
		sent.insert(i, VecDeque::new());
	}

    for _ in 0..ROUNDS {
		for (i, monkey) in monkeys.iter_mut().enumerate() {
			while let Some(item) = sent.get_mut(&i).unwrap().pop_front() {
				monkey.push(item);
			}
	
			while let Some(item) = monkey.pop() {
				monkey.play_count += 1;
				let worry_level = monkey.op.run(item) / 3;
				let destiny = if worry_level % monkey.test == 0 {
					monkey.destiny_if_true
				} else {
					monkey.destiny_if_false
				};
				sent.get_mut(&destiny).unwrap().push_back(worry_level);
			}
		}
	}

	monkeys.sort_by(|a, b| b.play_count.cmp(&a.play_count));
    Some(monkeys[0].play_count * monkeys[1].play_count)
}

pub fn part_two(input: Input) -> Solution {
    None
}

fn main() -> AocResult<()> {
    let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
        parse_monkeys(&input)
    };

    let mut input = advent_of_code::load!(setup)?;
    advent_of_code::solve!(1, part_one, &mut input);
    advent_of_code::solve!(2, part_two, &mut input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let mut input = parse_monkeys(&input).unwrap();
        assert_eq!(part_one(&mut input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let mut input = parse_monkeys(&input).unwrap();
        assert_eq!(part_two(&mut input), None);
    }
}
