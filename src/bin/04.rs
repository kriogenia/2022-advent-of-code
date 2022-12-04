const RANGE_SEPARATOR: &str = ",";
const SECTION_SEPARATOR: &str = "-"; 

struct SectionRange {
    from: i32,
    to: i32,
}

impl SectionRange {
	fn contains(&self, other: &SectionRange) -> bool {
		self.from <= other.from && self.to >= other.to
	}
}

impl From<&str> for SectionRange {
    fn from(value: &str) -> Self {
		let split_point = value.find(SECTION_SEPARATOR).expect("Ranges to have two sections separated by '-'");
		SectionRange {
			from: str::parse(&value[..split_point]).expect("Section to be an int"),
			to: str::parse(&value[split_point+1..]).expect("Section to be an int"),
		}
    }
}

fn split_line(line: &str) -> (&str, &str) {
	let split_point = line.find(RANGE_SEPARATOR).expect("Line to contain two ranges separated by ','");
	(&line[..split_point], &line[split_point + 1..])
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
		.map(split_line)
		.map(|(l, r)| (SectionRange::from(l), SectionRange::from(r)))
		.filter(|(l, r)| l.contains(r) || r.contains(l))
		.count() as u32;
    return Some(count);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
