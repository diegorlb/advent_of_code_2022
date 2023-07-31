use std::fs;

fn get_range_from(pair: &str) -> (i32, i32) {
  let mut pair = pair.split("-").map(|item| item.parse::<i32>().unwrap());
  let left = pair.next().unwrap();
  let right = pair.next().unwrap();

  return (left, right);
}

pub fn run_day_04_part_1() {
  match fs::read_to_string("./src/day_04/input") {
    Ok(input) => {
      let mut counter = 0;
      for line in input.lines() {
        let mut pairs = line.split(",");

        let (a_left, a_right) = get_range_from(pairs.next().unwrap());
        let (b_left, b_right) = get_range_from(pairs.next().unwrap());

        let is_a_inside_b = a_left <= b_left && b_right <= a_right;
        let is_b_inside_a = b_left <= a_left && a_right <= b_right;

        if is_a_inside_b || is_b_inside_a {
          counter += 1;
        }
      }

      println!("pairs: {}", counter);
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_04_part_2() {
  match fs::read_to_string("./src/day_04/input") {
    Ok(input) => {
      let mut counter = 0;
      for line in input.lines() {
        let mut pairs = line.split(",");

        let (a_left, a_right) = get_range_from(pairs.next().unwrap());
        let (b_left, b_right) = get_range_from(pairs.next().unwrap());

        let left_overlap = a_left <= b_left && b_left <= a_right;
        let right_overlap = a_left <= b_right && b_right <= a_right;

        let is_a_inside_b = a_left <= b_left && b_right <= a_right;
        let is_b_inside_a = b_left <= a_left && a_right <= b_right;

        if left_overlap || right_overlap || is_a_inside_b || is_b_inside_a {
          counter += 1;
        }
      }

      println!("pairs: {}", counter);
    }
    Err(error) => panic!("error: {}", error),
  };
}
