use advent_of_code::helpers::{AocResult, Folder};
use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc, str::FromStr};

const DAY: u8 = 7;
type Input<'a> = &'a Element;
type Solution = Option<u32>;

type ElementCell = Rc<RefCell<Element>>;

const THRESHOLD: u32 = 100_000;

pub enum Line {
    CdBack,
    Cd(String),
    Ls,
    Dir(String),
    File { size: u32, name: String },
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> AocResult<Self> {
        let mut splits = s.split_whitespace();
        match splits.next() {
            Some("$") => match splits.next() {
                Some("ls") => Ok(Line::Ls),
                Some("cd") => match splits.next() {
                    Some("..") => Ok(Line::CdBack),
                    Some(destination) => Ok(Line::Cd(destination.to_owned())),
                    None => Err("cd command missing destination".to_string()),
                },
                None | Some(_) => Err(format!("unknown command: {s}")),
            },
            Some("dir") => match splits.next() {
                Some(name) => Ok(Line::Dir(name.to_owned())),
                None => Err("dir output missing directory name".to_string()),
            },
            Some(size) => match (size.parse(), splits.next()) {
                (Ok(size), Some(name)) => Ok(Line::File {
                    size,
                    name: name.to_owned(),
                }),
                (Err(_), _) => Err(format!("file output with invalid size: {s}")),
                (Ok(_), None) => Err(format!("file output missing name: {s}")),
            },
            None => Err("unexpected empty line".to_string()),
        }
    }
}

pub struct Element {
    name: String,
    class: Class,
}

pub enum Class {
    Directory(Vec<ElementCell>),
    File(u32),
}

macro_rules! wrap {
    ($element:tt) => {
        Rc::new(RefCell::new($element))
    };
}

macro_rules! pwd {
    ($path_stack:tt) => {
        $path_stack
            .last()
            .expect("stack to never be empty")
            .deref()
            .borrow_mut()
    };
}

impl Element {
    fn mkdir(&mut self, name: &str) {
        if let Class::Directory(content) = self.class.borrow_mut() {
            let dir = Element {
                name: name.to_string(),
                class: Class::Directory(Vec::new()),
            };
            content.push(wrap!(dir))
        }
    }

    fn mk(&mut self, name: &str, size: u32) {
        if let Class::Directory(content) = self.class.borrow_mut() {
            let file = Element {
                name: name.to_string(),
                class: Class::File(size),
            };
            content.push(wrap!(file));
        }
    }

    fn get(&mut self, name: &str) -> Option<ElementCell> {
        if let Class::Directory(content) = self.class.borrow_mut() {
            if let Some(element) = content.iter().find(|&e| e.deref().borrow().name == name) {
                return Some(element.clone());
            }
        }
        None
    }

    fn size(&self) -> u32 {
        match &&self.class {
            Class::File(size) => *size,
            Class::Directory(content) => content.iter().map(|e| e.deref().borrow().size()).sum(),
        }
    }

    fn depth_first_size(&self, folder_filter: &impl Fn(u32) -> bool) -> u32 {
        if let Class::Directory(content) = &self.class {
            let sum = content
                .iter()
                .map(|e| e.deref().borrow().depth_first_size(folder_filter))
                .sum();
            let size = self.size();
            if folder_filter(size) {
                sum + size
            } else {
                sum
            }
        } else {
            0
        }
    }
}

fn build_filesystem(input: &[Line]) -> Element {
    let root = Element {
        name: "/".to_string(),
        class: Class::Directory(Vec::new()),
    };
    let root = wrap!(root);
    let mut path_stack = vec![root.clone()];

    for line in input.iter().skip(2) {
        match line {
            Line::CdBack => {
                path_stack.pop();
            }
            Line::Cd(destination) => {
                let next = pwd!(path_stack)
                    .get(destination)
                    .expect("dir to be stored in the current parent");
                path_stack.push(next);
            }
            Line::Dir(name) => {
                pwd!(path_stack).mkdir(name);
            }
            Line::File { size, name } => pwd!(path_stack).mk(name, *size),
            _ => {}
        };
    }
    drop(path_stack);

	root.replace(Element {
        name: "/".to_string(),
        class: Class::Directory(Vec::new()),
    })	// this almost looks like a hack but it works, it's like the start of the first Indiana Jones
}

pub fn part_one(input: Input) -> Solution {
    let filter = |s| s < THRESHOLD;
    let sum = input.depth_first_size(&filter);
    Some(sum)
}

pub fn part_two(input: Input) -> Solution {
	None
}

fn main() -> AocResult<()> {
    let input = &advent_of_code::helpers::read_input(Folder::Inputs, DAY)?;
    let file_tree = build_filesystem(input);
    advent_of_code::solve!(1, part_one, &file_tree);
    advent_of_code::solve!(2, part_two, &file_tree);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
		let file_tree = build_filesystem(input);
        assert_eq!(part_one(&file_tree), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::helpers::read_input(Folder::Examples, DAY).unwrap();
		let file_tree = build_filesystem(input);
        assert_eq!(part_two(&file_tree), None);
    }
}
