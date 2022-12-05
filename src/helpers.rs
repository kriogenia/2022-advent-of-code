use std::str::FromStr;

pub type AocResult<T> = Result<T, String>;

/// Reads the input and returns it as a collection of the mapped type
pub fn read_input<I>(folder: &str, day: u8) -> AocResult<Vec<I>>
where
    I: FromStr<Err = String>,
{
    super::read_file(folder, day)
        .lines()
        .map(I::from_str)
        .collect::<AocResult<Vec<I>>>()
}