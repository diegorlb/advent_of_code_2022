use std::fs;

pub fn run_day_06_part_1() {
  match fs::read_to_string("./src/day_06/input") {
    Ok(input) => {
      for line in input.lines() {
        let mut characters = line.chars();
        let mut window = Vec::new();
        let mut index = 0;

        while let Some(current) = characters.next() {
          if window.contains(&current) {
            let stop = match window[..] {
              [_, _, _, d, ..] if d == current => 4,
              [_, _, c, ..] if c == current => 3,
              [_, b, ..] if b == current => 2,
              [a, ..] if a == current => 1,

              _ => unreachable!(),
            };

            window.splice(0..stop, []);
          }

          window.push(current);
          index += 1;

          if window.len() == 4 {
            break;
          }
        }

        println!("output: {:?} at {}", window, index);
      }
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_06_part_2() {
  match fs::read_to_string("./src/day_06/input") {
    Ok(input) => {
      for line in input.lines() {
        let mut characters = line.chars();
        let mut window = Vec::new();
        let mut index = 0;

        while let Some(current) = characters.next() {
          if window.contains(&current) {
            let mut stop = 0;
            window.iter().enumerate().rev().any(|(index, repeated)| {
              if &current == repeated {
                stop = index;
                return true;
              }

              return false;
            });

            window.splice(0..=stop, []);
          }

          window.push(current);
          index += 1;

          if window.len() == 14 {
            break;
          }
        }

        println!("output: {:?} at {}", window, index);
      }
    }
    Err(error) => panic!("error: {}", error),
  };
}
