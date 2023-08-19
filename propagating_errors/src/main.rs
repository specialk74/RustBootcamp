use std::{io::{self, Read}, fs::File};

fn main() {
    let contents = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, io::Error>{
  let mut contents = String::new();
  File::open(filename)?.read_to_string(&mut contents)?;
  Ok(contents)
}

struct User {
    first_name: String,
    last_name: String,
}

fn get_initial(user: User) -> Option<String> {
    let first_initial = user.first_name.chars().next()?;
    let last_initial = user.last_name.chars().next()?;
    Some(format!("{}{}", first_initial, last_initial))
}
