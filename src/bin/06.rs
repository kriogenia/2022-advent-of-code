use advent_of_code::helpers::{AocResult, Folder};

const DAY: u8 = 6;
type Input<'a> = &'a [u8];
type Solution = Option<usize>;

// Marker for Part One
mod part_one {
    use crate::Marker;

    const SIZE: usize = 4;
    const ELEMENTS: usize = SIZE - 1;
    type Array = [u8; ELEMENTS * 2];

    pub struct MarkerOne(Array);

    impl MarkerOne {
        /// Inits the marker with the first three elements, it checks if the elements are repeated creating a Marker in valid state.
        pub fn init(starting: &[u8]) -> Self {
            let mut marker = Self([0; ELEMENTS * 2]);
            for b in &starting[..ELEMENTS] {
                marker.push(*b);
            }
            marker
        }
    }

    impl Marker for MarkerOne {
        fn advance(&mut self, positions: usize) {
            let mut cached = [0; ELEMENTS];
            cached.copy_from_slice(&self.0[positions..ELEMENTS + positions]);
            self.0[..ELEMENTS].copy_from_slice(&cached)
        }
        fn content(&mut self) -> &mut [u8] {
            &mut self.0
        }
        fn size(&self) -> usize {
            SIZE
        }
        fn elements(&self) -> usize {
            ELEMENTS
        }
    }
}

// Marker for Part Two
mod part_two {
    use crate::Marker;

    const SIZE: usize = 14;
    const ELEMENTS: usize = SIZE - 1;
    type Array = [u8; ELEMENTS * 2];

    pub struct MarkerTwo(Array);

    impl MarkerTwo {
        /// Inits the marker with the first three elements, it checks if the elements are repeated creating a Marker in valid state.
        pub fn init(starting: &[u8]) -> Self {
            let mut marker = Self([0; ELEMENTS * 2]);
            for b in &starting[..ELEMENTS] {
                marker.push(*b);
            }
            marker
        }
    }

    impl Marker for MarkerTwo {
        fn advance(&mut self, positions: usize) {
            let mut cached = [0; ELEMENTS];
            cached.copy_from_slice(&self.0[positions..ELEMENTS + positions]);
            self.0[..ELEMENTS].copy_from_slice(&cached)
        }
        fn content(&mut self) -> &mut [u8] {
            &mut self.0
        }
        fn size(&self) -> usize {
            SIZE
        }
        fn elements(&self) -> usize {
            ELEMENTS
        }
    }
}

/// Marker to find the code
trait Marker {
    /// Advances the elements n positions
    fn advance(&mut self, positions: usize);
    fn content(&mut self) -> &mut [u8];
    fn elements(&self) -> usize;
    fn size(&self) -> usize;

    /// Tries to add the value to the array, if it fails (the array is full) then return false, otherwise returns true
    fn add(&mut self, new: u8) -> bool {
        for i in 0..self.size() - 1 {
            if self.content()[i] == 0 {
                self.content()[i] = new;
                return true;
            }
        }
        false
    }

    /// Send the byte to the marker, it returns `true` if the push was succesful and false otherwise
    /// The push can fail if the Marker is full and there's no value equal to the given one
    fn push(&mut self, new: u8) -> bool {
        for i in (0..self.size()).rev() {
            if self.content()[i] == new {
                self.advance(i + 1);
                break;
            }
        }
        self.add(new)
    }
}

fn find_mark(input: Input, marker: &mut impl Marker) -> Solution {
    for (i, b) in input[marker.elements()..].iter().enumerate() {
        if !marker.push(*b) {
            return Some(i + marker.size());
        }
    }
    None
}

pub fn part_one(input: Input) -> Solution {
    let mut marker = part_one::MarkerOne::init(input);
    find_mark(input, &mut marker)
}

pub fn part_two(input: Input) -> Solution {
    let mut marker = part_two::MarkerTwo::init(input);
    find_mark(input, &mut marker)
}

fn main() -> AocResult<()> {
    let input = advent_of_code::read_file(Folder::Inputs, DAY);
    advent_of_code::solve!(1, part_one, input.as_bytes());
    advent_of_code::solve!(2, part_two, input.as_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part_one::MarkerOne;

    use super::*;

    const EXTRA_EXAMPLES: [(&[u8], usize, usize); 4] = [
        ("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 11, 26),
    ];

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        assert_eq!(part_one(&input.as_bytes()), Some(7));
        for (input, expected, _) in EXTRA_EXAMPLES {
            assert_eq!(part_one(&input), Some(expected));
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file(Folder::Examples, DAY);
        assert_eq!(part_two(&input.as_bytes()), Some(19));
        for (input, _, expected) in EXTRA_EXAMPLES {
            assert_eq!(part_two(&input), Some(expected));
        }
    }

    #[test]
    fn test_marker_push() {
        let mut marker = MarkerOne::init(&[b'a', b'b', b'c']);
        assert!(marker.push(b'b'));
        assert_eq!([b'c', b'b', 0, 0, 0, 0], marker.content());
        assert!(marker.push(b'b'));
        assert_eq!([b'b', 0, 0, 0, 0, 0], marker.content());
        assert!(marker.push(b'c'));
        assert!(marker.push(b'd'));
        assert_eq!([b'b', b'c', b'd', 0, 0, 0], marker.content());
        assert!(!marker.push(b'e'));
        assert_eq!([b'b', b'c', b'd', 0, 0, 0], marker.content());
    }

    #[test]
    fn test_marker_advance() {
        let mut marker = MarkerOne::init(&[b'm', b'j', b'q']);
        marker.advance(1);
        assert_eq!([b'j', b'q', 0, 0, 0, 0], marker.content());
        marker.advance(1);
        assert_eq!([b'q', 0, 0, 0, 0, 0], marker.content());
        let mut marker = MarkerOne::init(&[b'm', b'j', b'q']);
        marker.advance(3);
        assert_eq!([0, 0, 0, 0, 0, 0], marker.content());
    }
}
