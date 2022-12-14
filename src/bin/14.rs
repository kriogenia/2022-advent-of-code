use advent_of_code::helpers::{AocResult, Folder};
use std::{cmp::Ordering, ops::RangeInclusive, fmt::Debug};

const DAY: u8 = 14;
type Input<'a> = (Grid, usize);
type Solution = Option<u32>;

const ORIGIN: Coords = Coords(500, 0);

const OFFSET_X: usize = 330;
const SIZE_X: usize = 340;
const SIZE_Y: usize = 167;

type Grid = Vec<Vec<Tile>>;

#[derive(Clone, Eq, PartialEq)]
pub enum Tile {
    Rock,
    Sand,
    Air,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Air
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "O"),
            Self::Air => write!(f, "·"),
        }
    }
}

struct HorizontalLine {
    x: RangeInclusive<usize>,
    y: usize,
}

impl HorizontalLine {
    fn iter(from: &Coords, to: &Coords) -> impl Iterator<Item = Coords> {
        Self {
            x: from.0..=to.0,
            y: from.1,
        }
    }
}

impl Iterator for HorizontalLine {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        self.x.next().map(|x| Coords(x, self.y))
    }
}
struct VerticalLine {
    x: usize,
    y: RangeInclusive<usize>,
}

impl VerticalLine {
    fn iter(from: &Coords, to: &Coords) -> impl Iterator<Item = Coords> {
        Self {
            x: from.0,
            y: from.1..=to.1,
        }
    }
}

impl Iterator for VerticalLine {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        self.y.next().map(|y| Coords(self.x, y))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Coords(usize, usize);

impl Coords {
    fn line(&self, other: &Self) -> Box<dyn Iterator<Item = Self>> {
        match (&self.0.cmp(&other.0), &self.1.cmp(&other.1)) {
            (Ordering::Greater, _) => Box::new(HorizontalLine::iter(other, self)),
            (Ordering::Less, _) => Box::new(HorizontalLine::iter(self, other)),
            (_, Ordering::Greater) => Box::new(VerticalLine::iter(other, self)),
            (_, Ordering::Less) => Box::new(VerticalLine::iter(self, other)),
            (_, _) => unreachable!("wrong line between {self:?} and {other:?}"),
        }
    }
}

impl TryFrom<&str> for Coords {
    type Error = String;

    fn try_from(value: &str) -> AocResult<Self> {
        let mut splits = value.split(',');
        let x = splits
            .next()
            .and_then(|x| x.parse().ok())
            .ok_or_else(|| format!("invalid coordinate {value}"))?;
        let y = splits
            .next()
            .and_then(|x| x.parse().ok())
            .ok_or_else(|| format!("invalid coordinate {value}"))?;
        Ok(Coords(x, y))
    }
}

fn parse_grid(input: &str) -> AocResult<(Grid, usize)> {
    let mut grid: Grid = vec![vec![Tile::Air; SIZE_Y]; SIZE_X];
	let mut bottom = 0;
    for line in input.lines() {
        let mut splits = line.split(" -> ");
        let mut from: Coords = splits
            .next()
            .and_then(|s| s.try_into().ok())
            .ok_or_else(|| format!("input missing start: {input}"))?;
        for to in splits {
            let to: Coords = to
                .try_into()
                .map_err(|_| format!("input missing row end: {input}"))?;
            for Coords(x, y) in from.line(&to) {
                grid[x - OFFSET_X][y] = Tile::Rock;
            }
			bottom = bottom.max(from.1).max(to.1);
            from = to;
        }
    }
    Ok((grid, bottom))
}

fn get_next(point: &Coords, grid: &Grid) -> Option<Coords> {
	let x = point.0 - OFFSET_X;
	let y = point.1 + 1;
	if grid[x][y] == Tile::Air {
		Some(Coords(point.0, y))
	} else if grid[x - 1][y] == Tile::Air {
		Some(Coords(point.0 - 1, y))
	} else if grid[x + 1][y] == Tile::Air {
		Some(Coords(point.0 + 1, y))
	} else {
		None
	}
}

fn run(mut grid: Grid, check_next: impl Fn(&Coords) -> bool, check_current: impl Fn(&Coords) -> bool) -> u32 {
	let mut current = ORIGIN;
	let mut placed = 0;

	'out: loop {
		current = if let Some(next) = get_next(&current, &grid) {
			if check_next(&next) {
				break 'out; 
			}
			next
		} else {
			if check_current(&current) {
				break 'out;
			}
			grid[current.0 - OFFSET_X][current.1] = Tile::Sand;
			placed += 1;
			ORIGIN
		}
	}
	placed
}

pub fn part_one((grid, bottom): Input) -> Solution {
	Some(run(grid, |n| n.1 >= bottom, |_| false))
}

pub fn part_two((mut grid, bottom): Input) -> Solution {
	let bottom = bottom + 2;
	for tile in grid.iter_mut() {
		tile[bottom] = Tile::Rock;
	}
	Some(run(grid, |_| false, |c| *c == ORIGIN) + 1)
}

fn main() -> AocResult<()> {
    let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
        parse_grid(&input)
    };

    let (grid, bottom) = advent_of_code::load!(setup)?;
    advent_of_code::solve!(1, part_one, (grid.clone(), bottom));
    advent_of_code::solve!(2, part_two, (grid, bottom));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
		let (grid, bottom) = parse_grid(&input).unwrap();
        assert_eq!(part_one((grid, bottom)), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
		let (grid, bottom) = parse_grid(&input).unwrap();
		assert_eq!(part_two((grid, bottom)), Some(93));
    }
}
