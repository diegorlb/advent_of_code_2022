use std::fs;

fn get_priority(item: &u8) -> u32 {
  return match item {
    b'a'..=b'z' => (item - b'a' + 1) as u32,
    b'A'..=b'Z' => (item - b'A' + 27) as u32,

    _ => unreachable!(),
  };
}

pub fn run_day_03_part_1() {
  match fs::read_to_string("./src/day_03/input") {
    Ok(input) => {
      let mut counter = 0;

      let mut repetitions = Vec::new();
      for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        first.bytes().for_each(|byte| {
          if second.contains(byte as char) && !repetitions.contains(&byte) {
            repetitions.push(byte);
          }
        });

        counter += repetitions
          .iter()
          .map(|item| get_priority(item))
          .sum::<u32>();

        repetitions.clear();
      }

      println!("{:?}", counter)
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_03_part_2() {
  match fs::read_to_string("./src/day_03/input") {
    Ok(input) => {
      let mut repetitions = Vec::new();
      for chunk in Vec::from_iter(input.lines()).chunks(3) {
        let first = chunk[0];

        first.bytes().any(|byte| {
          if chunk[1].contains(byte as char) && chunk[2].contains(byte as char) {
            repetitions.push(byte);
            return true;
          }

          return false;
        });
      }

      let sum = repetitions
        .iter()
        .map(|repetition| get_priority(repetition))
        .sum::<u32>();

      println!("{:?}", sum);
    }
    Err(error) => panic!("error: {}", error),
  };
}
