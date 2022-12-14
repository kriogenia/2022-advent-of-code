use advent_of_code::helpers::{AocResult, Folder};
use std::{
    collections::{BinaryHeap, HashMap},
    mem,
};

const DAY: u8 = 12;
type Input<'a> = &'a ([[u8; GRID_WIDTH]; GRID_HEIGHT], Position, Position);
type Solution = Option<Distance>;
type Grid = [[u8; GRID_WIDTH]; GRID_HEIGHT];
type Position = (usize, usize);
type Distance = u16;

#[cfg(test)]
const GRID_WIDTH: usize = 8;
#[cfg(not(test))]
const GRID_WIDTH: usize = 136;
#[cfg(test)]
const GRID_HEIGHT: usize = 5;
#[cfg(not(test))]
const GRID_HEIGHT: usize = 41;

#[derive(Clone, Copy, Debug, Eq)]
struct Vertex {
    position: Position, //(line, column)
    distance: Distance,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

struct VertexHeap {
    heap: BinaryHeap<Vertex>,
}

impl VertexHeap {
    fn new() -> Self {
        Self {
            heap: Default::default(),
        }
    }

    fn pop(&mut self) -> Option<Vertex> {
        self.heap.pop()
    }

    fn push(&mut self, item: Vertex) {
        if !self.heap.iter().any(|v| v.position == item.position) {
            self.heap.push(item)
        }
    }

	/// Attempts to add all the positions to the heap, if they're already present it just updates them
	/// if the new distance is better
    fn push_all(&mut self, distance: Distance, positions: Vec<Position>) {
        let mut temp_heap = BinaryHeap::new();
        mem::swap(&mut temp_heap, &mut self.heap);
        for mut vertex in temp_heap.into_iter() {
            if positions.contains(&vertex.position) && distance < vertex.distance {
                vertex.distance = distance;
            }
            self.heap.push(vertex);
        }
        for position in positions {
            self.push(Vertex { position, distance })
        }
    }
}

fn parse_grid(input: &str) -> (Grid, Position, Position) {
    let mut grid: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [[0; GRID_WIDTH]; GRID_HEIGHT];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.bytes().enumerate() {
            grid[i][j] = c;
        }
    }
    let mut from = (0, 0);
    let mut to = (0, 0);
    for (i, lines) in grid.iter_mut().enumerate() {
        for (j, cell) in lines.iter_mut().enumerate() {
            match cell {
                b'S' => {
                    *cell = b'a';
                    from = (i, j);
                }
                b'E' => {
                    *cell = b'z';
                    to = (i, j);
                }
                _ => {}
            }
        }
    }
    (grid, from, to)
}

fn next_steps_up((i, j): Position, grid: &Grid) -> Vec<Position> {
    let mut neighbours = Vec::new();
    let max_height = grid[i][j] + 1;
    if i > 0 && grid[i - 1][j] <= max_height {
        neighbours.push((i - 1, j));
    }
    if i < GRID_HEIGHT - 1 && grid[i + 1][j] <= max_height {
        neighbours.push((i + 1, j));
    }
    if j > 0 && grid[i][j - 1] <= max_height {
        neighbours.push((i, j - 1));
    }
    if j < GRID_WIDTH - 1 && grid[i][j + 1] <= max_height {
        neighbours.push((i, j + 1));
    }
    neighbours
}

fn next_steps_down((i, j): Position, grid: &Grid) -> Vec<Position> {
    let mut neighbours = Vec::new();
    let min_height = grid[i][j] - 1;
    if i > 0 && grid[i - 1][j] >= min_height {
        neighbours.push((i - 1, j));
    }
    if i < GRID_HEIGHT - 1 && grid[i + 1][j] >= min_height {
        neighbours.push((i + 1, j));
    }
    if j > 0 && grid[i][j - 1] >= min_height {
        neighbours.push((i, j - 1));
    }
    if j < GRID_WIDTH - 1 && grid[i][j + 1] >= min_height {
        neighbours.push((i, j + 1));
    }
    neighbours
}

fn solve(
    grid: &Grid,
    start: Position,
    check: impl Fn(Position) -> bool,
    next_steps: impl Fn(Position, &Grid) -> Vec<Position>,
) -> Solution {
    let mut traveled = HashMap::new();
    let mut to_travel = VertexHeap::new();

    to_travel.push(Vertex {
        position: start,
        distance: 0,
    });

    loop {
        let Vertex { position, distance } = to_travel.pop().unwrap();
        traveled.insert(position, distance);
        if check(position) {
            return Some(distance);
        }

        let following_distance = distance + 1;
        let neighbours = next_steps(position, grid)
            .into_iter()
            .filter(|n| !traveled.contains_key(n))
            .collect();
        to_travel.push_all(following_distance, neighbours);
    }
}

pub fn part_one((grid, s, e): Input) -> Solution {
    solve(grid, *s, |position| position == *e, next_steps_up)
}

pub fn part_two((grid, _, e): Input) -> Solution {
    solve(
        grid,
        *e,
        |position| grid[position.0][position.1] == b'a',
        next_steps_down,
    )
}

fn main() -> AocResult<()> {
    let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
        let input = parse_grid(&input);
        Ok(input)
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
        let input = parse_grid(&input);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let input = parse_grid(&input);
        assert_eq!(part_two(&input), Some(29));
    }
}
