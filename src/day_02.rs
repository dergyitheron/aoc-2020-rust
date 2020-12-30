use itertools::Itertools;
use std::time::{Duration, Instant};

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
      .map(|line| {
        line
          .split(|item| item == '-' || item == ' ' || item == ':')
          .collect_vec()
      })
      .filter(|item| {
        let count = item[4].matches(item[2]).collect::<Vec<&str>>().len();
        count >= item[0].parse().unwrap() && count <= item[1].parse().unwrap()
      })
      .count();

    //  println!("{}", result);

    acc += Instant::now().duration_since(start);
  }
  acc / loops
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
      .map(|line| {
        line
          .split(|item| item == '-' || item == ' ' || item == ':')
          .collect_vec()
      })
      .filter(|item| {
        let characters = item[4].chars().collect_vec();
        let first_pos = characters[item[0].parse::<usize>().unwrap() - 1];
        let second_pos = characters[item[1].parse::<usize>().unwrap() - 1];
        let letter = item[2].parse().unwrap();

        (first_pos == letter) ^ (second_pos == letter)
      })
      .count();

    println!("{}", _result);

    acc += Instant::now().duration_since(start);
  }
  acc / loops
}
