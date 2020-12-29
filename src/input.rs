use std::fs;
use std::env;

pub fn file_to_string(filepath: &str) -> String {
  let mut path = env::current_dir().expect("Error reading current directory");
  path.push(filepath);
  fs::read_to_string(path).expect("Something went wrong reading the file")
}
