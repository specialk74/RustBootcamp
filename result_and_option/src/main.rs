use std::{fs, io};

fn main() {
    let first_line = read_first_line("example.txt");
}

fn read_first_line(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
}
