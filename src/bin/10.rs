use advent_of_code::helpers::{AocResult, Folder};
use std::str::FromStr;

const DAY: u8 = 10;
type Input<'a> = &'a [Instruction];
type Solution = Option<i16>;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i16),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        match s {
            "noop" => Ok(Self::Noop),
            _ => Ok(Self::Addx(
                s.split_at(5)
                    .1
                    .parse()
                    .map_err(|_| format!("Invalid instruction {s}"))?,
            )),
        }
    }
}

struct Cpu {
    pc: u16,
    vx: i16,
    busy: u16,
    to_add: i16,
    signal_strength: i16,
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0,
            vx: 1,
            busy: 0,
            to_add: 0,
            signal_strength: 0,
        }
    }

    fn run(&mut self, instructions: Input) {
        let mut iter = instructions.iter();
        loop {
            self.pc += 1;
            if self.pc % 40 == 20 {
                self.signal_strength += self.vx * self.pc as i16
            }

            //println!("Cycle {}, X: {}", self.pc, self.vx);
            match self.busy {
                0 => match iter.next() {
                    Some(instruction) => self.execute(instruction),
                    None => return,
                },
                1 => self.awake(),
                _ => unreachable!("max busy is 1"), // todo change to bool?
                                                    //2.. => self.sleep()
            }
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        //println!("Run: {instruction:?}");
        match instruction {
            Instruction::Noop => {
                self.to_add = 0;
            }
            Instruction::Addx(x) => {
                self.busy = 1;
                self.to_add = *x;
            }
        };
    }

    fn awake(&mut self) {
        self.busy = 0;
        self.vx += self.to_add;
        //println!("Awakening. Adding {}. X: {}", self.to_add, self.vx);
    }
    /*
        fn sleep(&mut self) {
            println!("Sleeping. Busy: {}", self.busy);
            self.busy -= 1;
        }
    */
}

pub fn part_one(input: Input) -> Solution {
    let mut cpu = Cpu::new();
    cpu.run(input);
    Some(cpu.signal_strength)
}

pub fn part_two(input: Input) -> Solution {
    None
}

fn main() -> AocResult<()> {
    let input = advent_of_code::helpers::read_input(Folder::Inputs, DAY)?;
    //let input = &advent_of_code::read_file("inputs", DAY);				// if you want just the string
    advent_of_code::solve!(1, part_one, &input);
    advent_of_code::solve!(2, part_two, &input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_cpu_cycle() {
        let instructions = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];

        let mut cpu = Cpu::new();
        cpu.run(&instructions);

        assert_eq!(6, cpu.pc);
        assert_eq!(0, cpu.busy);
    }
}
