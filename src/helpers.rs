use std::str::FromStr;

pub type AocResult<T> = Result<T, String>;

pub const INPUT_FOLDER: &str = "inputs";
pub const TEST_FOLDER: &str = "examples";

pub enum Folder {
    Inputs,
    Examples,
}

impl Folder {
    pub(crate) fn path(&self) -> &str {
        match self {
            Self::Inputs => INPUT_FOLDER,
            Self::Examples => TEST_FOLDER,
        }
    }
}

/// Reads the input and returns it as a collection of the mapped type
pub fn read_input<I>(folder: Folder, day: u8) -> AocResult<Vec<I>>
where
    I: FromStr<Err = String>,
{
    super::read_file(folder, day)
        .lines()
        .map(str::parse)
        .collect::<AocResult<Vec<I>>>()
}

/// Parses the input from an already loaded String, needed for lines keeping references to the string
pub fn parse_input<'a, I>(input: &'a str) -> AocResult<Vec<I>>
where
	I: TryFrom<&'a str, Error = String>
{
    input.lines().map(TryFrom::try_from).collect::<AocResult<Vec<I>>>()
}
