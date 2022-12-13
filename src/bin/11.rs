use std::{str::FromStr, collections::HashMap};

use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 11;
type Input<'a> = &'a mut [Monkey];
type Solution = Option<usize>;
type Ux = u64;

const STARTING_ITEMS_LIST_POINTER: usize = 18;
const OP_DECLARATION_POINTER: usize = 19;
const TEST_NUMBER_POINTER: usize = 21;
const IF_TRUE_INDEX_POINTER: usize = 29;
const IF_FALSE_INDEX_POINTER: usize = 30;

#[derive(Clone, Debug)]
pub struct Monkey {
    items: Vec<Ux>,
    op: Operation,
    test: Ux,
    on_ok: usize,
    on_fail: usize,
	play_count: usize,
}

impl Monkey {
	fn push(&mut self, item: Ux) {
		self.items.push(item);
	}
}

#[derive(Clone, Debug)]
enum Operation {
    Sum(Value, Value),
    Mul(Value, Value),
}

impl Operation {
    fn run(&self, value: Ux) -> Ux {
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

#[derive(Clone, Debug)]
enum Value {
    Old,
    Literal(Ux),
}

impl Value {
    fn get(&self, or: Ux) -> Ux {
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
            None => Err("missing".to_string()),
        }
    }
}

fn parse_monkeys(input: &str) -> AocResult<Vec<Monkey>> {
    let mut monkeys = Vec::new();
    let mut iter = input.lines();
    loop {
        if iter.next().is_none() {
            break; // monkeys are declared sequentially so we don't care about their hread
        }
        let items = parse_items(
            iter.next()
                .ok_or_else(|| "Missing starting items line".to_string())?,
        )?;
        let op = iter
            .next()
            .ok_or_else(|| "Missing operation line".to_string())?
            .try_into()?;
        let test = parse_number(iter.next(), TEST_NUMBER_POINTER)
            .map_err(|e| format!("Invalid test line: {e}"))?;
        let on_ok = parse_number(iter.next(), IF_TRUE_INDEX_POINTER)
            .map_err(|e| format!("Invalid condition true line: {e}"))?;
        let on_fail = parse_number(iter.next(), IF_FALSE_INDEX_POINTER)
            .map_err(|e| format!("Invalid condition false line: {e}"))?;
        monkeys.push(Monkey {
            items,
            op,
            test,
            on_ok,
            on_fail,
			play_count: 0,
        });
        iter.next(); // jump empty line
    }
    Ok(monkeys)
}

fn parse_items(line: &str) -> AocResult<Vec<Ux>> {
    let mut items = Vec::new();
    for value in line.split_at(STARTING_ITEMS_LIST_POINTER).1.split(", ") {
        items.push(
            value
                .parse()
                .map_err(|_| format!("invalid item: {value}"))?,
        )
    }
    Ok(items)
}

fn parse_number<T: FromStr>(line: Option<&str>, split: usize) -> AocResult<T> {
    line.map(|s| s.split_at(split).1)
        .ok_or_else(|| "missing".to_string())?
        .parse()
        .map_err(|_| "failed parsing".to_string())
}

fn solve<const ROUNDS: usize, const RELIEF: Ux>(monkeys: Input) -> Solution {
	let mut sent = HashMap::new();
	for i in 0..monkeys.len() {
		sent.insert(i, Vec::new());
	}

	let mcd: Ux = monkeys.iter().map(|m| m.test).product();

    for _ in 0..ROUNDS {
		for (i, monkey) in monkeys.iter_mut().enumerate() {
			 for item in sent.get_mut(&i).unwrap().drain(..) {
				monkey.push(item);
			}
	
			monkey.play_count += monkey.items.len();
			for item in monkey.items.drain(..) {
				let worry_level = (monkey.op.run(item) % mcd) / RELIEF;
				let destiny = if worry_level % monkey.test == 0 {
					monkey.on_ok
				} else {
					monkey.on_fail
				};
				sent.get_mut(&destiny).unwrap().push(worry_level);
			}
		}
	}

	monkeys.sort_by(|a, b| b.play_count.cmp(&a.play_count));
    Some(monkeys[0].play_count * monkeys[1].play_count)
}

pub fn part_one(input: Input) -> Solution {
	solve::<20, 3>(input)
}

pub fn part_two(input: Input) -> Solution {
    solve::<10_000, 1>(input)
}

fn main() -> AocResult<()> {
    let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
        let monkeys = parse_monkeys(&input)?;
		Ok((monkeys.clone(), monkeys))
    };

    let mut input = advent_of_code::load!(setup)?;
    advent_of_code::solve!(1, part_one, &mut input.0);
    advent_of_code::solve!(2, part_two, &mut input.1);
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
        assert_eq!(part_two(&mut input), Some(2713310158));
    }
}
