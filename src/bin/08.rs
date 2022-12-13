use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 8;
type Solution = Option<u32>;

#[cfg(test)]
const GRID_SIZE: usize = 5;
#[cfg(not(test))]
const GRID_SIZE: usize = 99; // handpicked size to work with arrays

type Grid<T> = [[T; GRID_SIZE]; GRID_SIZE];
const LAST_ELEMENT: usize = GRID_SIZE - 1;

const MAX_HEIGHT: u8 = b'9';
const HEIGHT_CHECK: u8 = MAX_HEIGHT * 2;

struct Counter {
    top_bottom: u8,
    left_right: u8,
    bottom_top: u8,
    right_left: u8,
}

impl Counter {
    fn new(heights: &Grid<u8>, offset: usize) -> Self {
        Self {
            top_bottom: heights[0][offset],
            left_right: heights[offset][0],
            bottom_top: heights[LAST_ELEMENT][offset],
            right_left: heights[offset][LAST_ELEMENT],
        }
    }
}

macro_rules! visibility {
    ($direction:ident -> $i:expr, $j:expr, $c: ident, $v:ident, $h:ident) => {
        if $h[$i][$j] > $c.$direction {
            $c.$direction = $h[$i][$j];
            $v[$i][$j] = true
        }
    };
}

fn init_grids(input: String) -> (Grid<u8>, Grid<bool>) {
    let mut height_grid = [[0; GRID_SIZE]; GRID_SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, height) in line.bytes().enumerate() {
            height_grid[i][j] = height;
        }
    }

    let mut visible_grid = [[false; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        visible_grid[0][i] = true;
        visible_grid[i][0] = true;
        visible_grid[LAST_ELEMENT][i] = true;
        visible_grid[i][LAST_ELEMENT] = true;
    }

    (height_grid, visible_grid)
}

fn get_score(heights: &Grid<u8>, (i, j): (usize, usize)) -> u32 {
    let height = heights[i][j];
    let top = score_backwards(i, |h| heights[*h][j] < height);
    let left = score_backwards(j, |h| heights[i][*h] < height);
    let bottom = score_forwards(i, |h| heights[*h][j] < height);
    let right = score_forwards(j, |h| heights[i][*h] < height);
    top * left * bottom * right
}

fn score_backwards(position: usize, height_check: impl Fn(&usize) -> bool) -> u32 {
    let to = position;
    let count = (0..position).rev().take_while(height_check).count();
    if count < to {
        count as u32 + 1
    } else {
        count as u32
    }
}

fn score_forwards(position: usize, height_check: impl Fn(&usize) -> bool) -> u32 {
    let count = (position + 1..GRID_SIZE).take_while(height_check).count();
    if count < (LAST_ELEMENT - position) {
        count as u32 + 1
    } else {
        count as u32
    }
}

pub fn part_one((heights, visibility): (&Grid<u8>, &mut Grid<bool>)) -> Solution {
    for i in 1..LAST_ELEMENT {
        let mut counter = Counter::new(heights, i);
        for j in 1..LAST_ELEMENT {
            visibility!(top_bottom -> j, i, counter, visibility, heights);
            visibility!(left_right -> i, j, counter, visibility, heights);
            if (counter.top_bottom + counter.left_right) >= HEIGHT_CHECK {
                break;
            }
        }
        for j in (1..LAST_ELEMENT).rev() {
            visibility!(bottom_top -> j, i, counter, visibility, heights);
            visibility!(right_left -> i, j, counter, visibility, heights);
            if (counter.bottom_top + counter.right_left) >= HEIGHT_CHECK {
                break;
            }
        }
    }

    let total = visibility
        .iter_mut()
        .flat_map(|x| x.iter_mut().filter(|x| **x))
        .count() as u32;
    Some(total)
}

pub fn part_two(heights: &Grid<u8>) -> Solution {
    let mut max = 0;
    for i in 1..LAST_ELEMENT {
        for j in 1..LAST_ELEMENT {
            let score = get_score(heights, (i, j));
            if score > max {
                max = score;
            }
        }
    }
    Some(max)
}

fn main() -> AocResult<()> {
	let setup = || {
        let input = advent_of_code::read_file(Folder::Inputs, DAY);
    	Ok(init_grids(input))
    };

    let (heights, mut visibility) = advent_of_code::load!(setup)?;
    advent_of_code::solve!(1, part_one, (&heights, &mut visibility));
    advent_of_code::solve!(2, part_two, &heights);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let (heights, mut visibility) = init_grids(input);
        assert_eq!(part_one((&heights, &mut visibility)), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        let (heights, _) = init_grids(input);
        assert_eq!(part_two(&heights), Some(8));
    }
}
