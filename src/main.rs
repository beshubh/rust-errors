use core::{error, fmt, num};
use std::error::Error;
use std::{
    env,
    fmt::Debug,
    fs::File,
    io::{self, Read},
    path::Path,
};

fn _guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("invalid number");
    }
    n == 5
}

fn _find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn _double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("please give atleast one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|i| i * 2)
}

fn _file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents: String| {
            contents
                .trim()
                .parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| n * 2)
}

fn file_double_ergo<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n = contents.trim().parse::<i32>()?;
    Ok(2 * n)
}

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::Io(value)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::Parse(value)
    }
}

fn main() {
    match file_double_ergo("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
