use std::time::{Instant, Duration};
use itertools::Itertools;

use super::input::file_to_string;

pub fn run() {
  let loops: u32 = 100;
  println!("part1_1: {:?}", benchmark(part1_1, loops));
  println!("part1_2: {:?}", benchmark(part1_2, loops));
  println!("part1_3: {:?}", benchmark(part1_3, loops));
  println!("part1_4: {:?}", benchmark(part1_4, loops));
  println!("part2_1: {:?}", benchmark(part2_1, loops));
}

// BENCHMARK FUNCTION
fn benchmark(alg: fn(&Vec<u32>) -> Option<(u32, u32)>, loops: u32) -> Duration {
  // Read input data
  let contents = file_to_string("input/day_01.txt");
  // Start measuring and calculating
  let mut acc = Duration::new(0, 0);
  for _i in 0..loops {
    let start = Instant::now();

    let data: Vec<u32> = contents
      .lines()
      .map(|line| line.parse().unwrap())
      .collect();

    alg(&data);
    acc += Instant::now().duration_since(start);
  }
  acc/loops
}

// PART 1
fn part1_1(data: &Vec<u32>) -> Option<(u32, u32)> {
  for n in data.iter() {
    for m in data.iter() {
      if n + m == 2020 { return Some((*n, *m)); }
    }
  }
  None
}

fn part1_2(data: &Vec<u32>) -> Option<(u32, u32)> {
  for i in 0..data.len() {
    for j in i..data.len() {
      if data[i] + data[j] == 2020 {return Some((data[i], data[j]))}
    }
  }
  None
}

fn part1_3(data: &Vec<u32>) -> Option<(u32, u32)> {
  for x in data.iter() {
    let y = 2020 - *x;
    if data.contains(&y) { return Some((*x, y)); }
  }
  None
}

fn part1_4(data: &Vec<u32>) -> Option<(u32, u32)> {
  let min_val = data.iter().min().unwrap();
  let max_val = data.iter().max().unwrap();
  let ok_vals = data.iter().filter(|x| ((2020 - *max_val) <= **x) && (**x <= (2020 - *min_val)));

  for i in ok_vals.combinations(2) {
    if i.iter().copied().sum::<u32>() == 2020 {return None;}
  }
  None
}

// PART 2
fn part2_1(data: &Vec<u32>) -> Option<(u32, u32)> {
  let min_val = data.iter().min().unwrap();

  for n in data.iter() {
    for m in data.iter() {
      if m + n > 2020 - min_val {continue;}
      for o in data.iter() {
        if (n + m + o) == 2020 { return Some((n*m*o, 1)); }
      }
    }
  }
  None
}
