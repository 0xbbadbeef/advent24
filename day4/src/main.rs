use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, 1),
  (0, -1),
  (1, -1),
  (1, 0),
  (1, 1),
];

fn get_word_at_direction(
  lines: &[&str],
  start: (i32, i32),
  direction: (i32, i32),
  word: &str,
) -> String {
  // protect against start of word being out of bounds
  if start.0 >= lines[0].len().try_into().unwrap()
    || start.0 < 0
    || start.1 >= lines.len().try_into().unwrap()
    || start.1 < 0
  {
    return "".to_string();
  };

  // protect against end of word being out of bounds
  let word_len: i32 = word.len() as i32;
  let x_bounds = start.0 + ((word_len - 1) * direction.0);
  let y_bounds = start.1 + ((word_len - 1) * direction.1);
  if x_bounds >= lines[0].len().try_into().unwrap()
    || x_bounds < 0
    || y_bounds >= lines.len().try_into().unwrap()
    || y_bounds < 0
  {
    return "".to_string();
  };

  let mut result = vec![];
  for distance in 0..word_len {
    let line_index: usize = (start.1 + (distance * direction.1)).try_into().unwrap();
    let char_index: usize = (start.0 + (distance * direction.0)).try_into().unwrap();

    let char = lines
      .get(line_index)
      .unwrap()
      .chars()
      .nth(char_index)
      .unwrap()
      .to_string();
    result.push(char);
  }

  result.join("")
}

fn part_one(lines: &[&str]) -> i32 {
  let target_word: &str = "XMAS";
  lines
    .iter()
    .enumerate()
    .fold(0, |mut acc, (line_index, line)| {
      line
        .chars()
        .enumerate()
        .for_each(|(char_index, line_char)| {
          if line_char != 'X' {
            return;
          }

          for direction in DIRECTIONS {
            let word = get_word_at_direction(
              lines,
              (
                char_index.try_into().unwrap(),
                line_index.try_into().unwrap(),
              ),
              direction,
              target_word,
            );
            if word == target_word {
              acc += 1;
            }
          }
        });

      acc
    })
}

fn part_two(lines: &[&str]) -> i32 {
  let word = "MAS";

  lines
    .iter()
    .enumerate()
    .fold(0, |mut acc, (line_index, line)| {
      line
        .chars()
        .enumerate()
        .for_each(|(char_index, line_char)| {
          if line_char != 'A' {
            return;
          }

          let mut mas_count = 0;
          for direction in [(-1, -1), (1, 1), (-1, 1), (1, -1)] {
            if get_word_at_direction(
              lines,
              (
                (char_index as i32) + direction.0,
                (line_index as i32) + direction.1,
              ),
              ((-direction.0).signum(), (-direction.1).signum()),
              word,
            ) == word
            {
              mas_count += 1;
            }
          }

          if mas_count >= 2 {
            acc += 1;
          }
        });

      acc
    })
}

fn main() {
  let input = fs::read_to_string("inputreal.txt").expect("Could not read input");

  let lines = input.lines().collect::<Vec<_>>();
  println!("part one: {:?}", part_one(&lines));
  println!("part two: {:?}", part_two(&lines));
}
