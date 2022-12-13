use advent_of_code::helpers::{AocResult, Folder};
use std::{io::Write, str::FromStr};

const DAY: u8 = 10;
type Input<'a> = &'a Device;
type Solution = Option<i16>;

const TOTAL_LINES: usize = 6;
const LINE_SIZE: usize = 40;
const TOTAL_CYCLES: usize = TOTAL_LINES * LINE_SIZE;

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

pub struct Device {
    pc: i16,
    vx: i16,
    sleep_flag: bool,
    to_add: i16,
    ssc: i16, // signal strength counter
    display_buffer: [u8; TOTAL_CYCLES],
}

impl Device {
    fn new() -> Self {
        Self {
            pc: 0,
            vx: 1,
            sleep_flag: false,
            to_add: 0,
            ssc: 0,
            display_buffer: [0; TOTAL_CYCLES],
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        let mut iter = instructions.iter();
        loop {
            if self.pc >= TOTAL_CYCLES as i16 {
                return;
            }

            self.display_buffer[self.pc as usize] = match self.pc % 40 - self.vx {
                -1..=1 => b'#',
                _ => b'.',
            };

            self.pc += 1;

            if self.pc % 40 == 20 {
                self.ssc += self.vx * self.pc
            }

            if self.sleep_flag {
                self.awake();
            } else if let Some(instruction) = iter.next() {
                self.execute(instruction);
            };
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.to_add = 0;
            }
            Instruction::Addx(x) => {
                self.sleep_flag = true;
                self.to_add = *x;
            }
        };
    }

    fn awake(&mut self) {
        self.sleep_flag = false;
        self.vx += self.to_add;
    }
}

pub fn part_one(cpu: Input) -> Solution {
    Some(cpu.ssc)
}

pub fn part_two(cpu: Input) -> Solution {
    let buffer = &cpu.display_buffer;
    let mut out = std::io::stdout();
    for i in 0..TOTAL_LINES {
        out.write_all(&buffer[i * LINE_SIZE..(i * LINE_SIZE + LINE_SIZE)])
            .unwrap();
        out.write_all(&[b'\n']).unwrap();
    }
    out.flush().unwrap();
    Some(1) // not the actual answer, placeholder to measure the execution
}

fn main() -> AocResult<()> {
    let input = advent_of_code::helpers::read_input(Folder::Inputs, DAY)?;

    let mut cpu = Device::new();
    cpu.run(&input);

    advent_of_code::solve!(1, part_one, &cpu);
    advent_of_code::solve!(2, part_two, &cpu);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
        let mut cpu = Device::new();
        cpu.run(&input);
        assert_eq!(part_one(&cpu), Some(13140));
    }
}
