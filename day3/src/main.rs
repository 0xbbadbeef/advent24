use regex::Regex;
use std::fs;

fn capture_mults(input: &str) -> Vec<(u32, u32)> {
  let matcher = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
  matcher
    .captures_iter(input)
    .map(|m| (m[1].parse::<u32>().unwrap(), m[2].parse::<u32>().unwrap()))
    .collect()
}

fn part_one(input: &str) {
  let part_one = capture_mults(input)
    .iter()
    .fold(0, |acc, (a, b)| acc + a * b);

  println!("part one {:?}", part_one);
}

fn part_two(input: &str) {
  let enabled_sections = input
    .split("do()")
    .map(|starts| starts.split("don't()").collect::<Vec<_>>()[0])
    .collect::<Vec<_>>()
    .join("\n");

  let part_two = capture_mults(&enabled_sections)
    .iter()
    .fold(0, |acc, (a, b)| acc + a * b);
  println!("part two {:?}", part_two);
}

fn main() {
  let input = fs::read_to_string("./inputreal.txt").expect("Could not read input");

  part_one(&input);
  part_two(&input);
}
