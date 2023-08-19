use std::{io, fs, error, num::ParseIntError};

fn main() {
    let i = parse_file("example.txt");
    let i = parse_file2("example.txt");
    let j = parse_file2("example.txt");

    match i {
        Ok(i) => println!("{:?}", i),
        Err(e) => {
            match e {
                ParseFileError::File => println!("File not found"),
                ParseFileError::Parse(e) => println!("IO error: {}", e),
            }
        }
    }

    match j {
        Ok(i) => println!("{:?}", i),
        Err(ParseFileError::File) => println!("File not found"),
        Err(ParseFileError::Parse(e)) => println!("IO error: {}", e),
    }
}

fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}
enum ParseFileError {
    File,
    Parse(ParseIntError)
}

fn parse_file2(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
        .map_err(|_| ParseFileError::File)?;
    let i = s.parse()
        .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

