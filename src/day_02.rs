use std::time::{Instant, Duration};
use itertools::Itertools;

use super::input::file_to_string;

pub fn run() {
  let loops: u32 = 100;
  println!("part1: {:?}", part1(loops));
  println!("part2: {:?}", part2(loops));
}

// PART 1
fn part1(loops: u32) -> Duration {
  // Read input data
  let contents = file_to_string("input/day_02.txt");

  // Start measuring and calculating
  let mut acc = Duration::new(0, 0);

  for _i in 0..loops {
    let start = Instant::now();

    let _result = contents
      .lines()
      .map(|line| line.split(|item| item == '-' || item == ' ' || item == ':').collect_vec())
      .filter(|item| {
        let count = item[4].matches(item[2]).collect::<Vec<&str>>().len();
        count >= item[0].parse().unwrap() && count <= item[1].parse().unwrap()
      }).count();

    //  println!("{}", result);

    acc += Instant::now().duration_since(start);
  }
  acc/loops
}

// PART 2
fn part2(loops: u32) -> Duration {
  // Read input data
  let contents = file_to_string("input/day_02.txt");

  // Start measuring and calculating
  let mut acc = Duration::new(0, 0);

  for _i in 0..loops {
    let start = Instant::now();

    let _result = contents
      .lines()
      .map(|line| line.split(|item| item == '-' || item == ' ' || item == ':').collect_vec())
      .filter(|item| {
        let characters = item[4].chars().collect_vec();
        (characters[item[0].parse::<usize>().unwrap()-1] == item[2].parse().unwrap())
          && characters[item[1].parse::<usize>().unwrap()-1] != item[2].parse().unwrap() ||
        (characters[item[0].parse::<usize>().unwrap()-1] != item[2].parse().unwrap()
          && characters[item[1].parse::<usize>().unwrap()-1] == item[2].parse().unwrap())
      }).count();

    //  println!("{}", _result);

    acc += Instant::now().duration_since(start);
  }
  acc/loops
}
