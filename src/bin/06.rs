use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 6;
type Input<'a> = &'a [u8];
type Solution = Option<usize>;

const MARKER_SIZE: usize = 4;
const SAVED_ELEMENTS: usize = MARKER_SIZE - 1;
const MEMORY_SIZE: usize = SAVED_ELEMENTS * 2;

struct Marker([u8; MEMORY_SIZE]);

impl Marker {
	/// Inits the marker with the first three elements, it's a naive implementation.
	/// It does not check if the three are different, it expects them to be. If they're not, early exit is possible.
	/// Use only when the input is known to be valid (my case)
	#[cfg(not(test))]
	fn init(starting: &[u8]) -> Self {
		if starting.len() != SAVED_ELEMENTS {
			panic!("the marker should only hold {SAVED_ELEMENTS} elements")
		}
		let mut elements = [0; MEMORY_SIZE];
		elements[..SAVED_ELEMENTS].copy_from_slice(starting);
		Self(elements)
	}

	/// Inits the marker with the first three elements, it checks if the elements are repeated creating a Marker in valid state.
	/// Required for testing as some of the examples would be fail otherwise
	#[cfg(test)]
	fn init(starting: &[u8]) -> Self {
		if starting.len() != SAVED_ELEMENTS {
			panic!("the marker should only hold {SAVED_ELEMENTS} elements")
		}
		let mut marker = Self([0; MEMORY_SIZE]);
		for b in starting {
			marker.push(*b);
		}
		marker
	}

	/// Send the byte to the marker, it returns `true` if the push was succesful and false otherwise
	/// The push can fail if the Marker is full and there's no value equal to the given one
	fn push(&mut self, new: u8) -> bool {
		for i in (0..MARKER_SIZE).rev() {
			if self.0[i] == new {
				self.advance(i + 1);
				break;
			}
		}
		self.add(new)
	}

	/// Tries to add the value to the array, if it fails (the array is full) then return false, otherwise returns true
	fn add(&mut self, new: u8) -> bool {
		for i in 0..SAVED_ELEMENTS {
			if self.0[i] == 0 {
				self.0[i] = new;
				return true;
			}
		}
		false
	}

	/// Advances the elements n positions
	fn advance(&mut self, positions: usize) {
		let mut cached = [0; SAVED_ELEMENTS];
		cached.copy_from_slice(&self.0[positions..SAVED_ELEMENTS + positions]);
		self.0[..SAVED_ELEMENTS].copy_from_slice(&cached)
	}

}

pub fn part_one(input: Input) -> Solution {
	let mut marker = Marker::init(&input[..3]);	

	for (i, b) in input[SAVED_ELEMENTS..].iter().enumerate() {
		if !marker.push(*b) {
			return Some(i + MARKER_SIZE);
		}
	}
	None
}

pub fn part_two(input: Input) -> Solution {
    None
}

fn main() -> AocResult<()> {
    let input = advent_of_code::read_file(Folder::Inputs, DAY);
    advent_of_code::solve!(1, part_one, input.as_bytes());
    advent_of_code::solve!(2, part_two, input.as_bytes());
	Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

	const EXTRA_EXAMPLES: [(&[u8], usize); 4] = [
		("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 5),
		("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 6),
		("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 10),
		("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 11),
	];

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        assert_eq!(part_one(&input.as_bytes()), Some(7));
		for (input, expected) in EXTRA_EXAMPLES {
			assert_eq!(part_one(&input), Some(expected));
		}
    }
	
    #[test]
    fn test_part_two() {
		let input = advent_of_code::read_file(Folder::Inputs, DAY);
        assert_eq!(part_two(&input.as_bytes()), None);
    }

	#[test]
	fn test_marker_init() {
		assert_eq!([b'm', b'j', b'q', 0, 0, 0], Marker::init(&[b'm', b'j', b'q']).0);
		assert_eq!([b'j', 0, 0, 0, 0, 0], Marker::init(&[b'm', b'j', b'j']).0);
	}

	#[test]
	fn test_marker_push() {
		let mut marker = Marker::init(&[b'a', b'b', b'c']);
		assert!(marker.push(b'b'));
		assert_eq!([b'c', b'b', 0, 0, 0, 0], marker.0);
		assert!(marker.push(b'b'));
		assert_eq!([b'b', 0, 0, 0, 0, 0], marker.0);
		assert!(marker.push(b'c'));
		assert!(marker.push(b'd'));
		assert_eq!([b'b', b'c', b'd', 0, 0, 0], marker.0);
		assert!(!marker.push(b'e'));
		assert_eq!([b'b', b'c', b'd', 0, 0, 0], marker.0);
		
	}

	#[test]
	fn test_marker_advance() {
		let mut marker = Marker::init(&[b'm', b'j', b'q']);
		marker.advance(1);
		assert_eq!([b'j', b'q', 0, 0, 0, 0], marker.0);
		marker.advance(1);
		assert_eq!([b'q', 0, 0, 0, 0, 0], marker.0);
		let mut marker = Marker::init(&[b'm', b'j', b'q']);
		marker.advance(3);
		assert_eq!([0, 0, 0, 0, 0, 0], marker.0);
	}

}
