use std::fs;

fn get_calories_per_elf() -> Vec<i32> {
  return match fs::read_to_string("./src/day_01/input") {
    Ok(input) => {
      let mut calories_per_elf = Vec::new();
      let mut sum = 0;
      for line in input.lines() {
        if line.is_empty() {
          calories_per_elf.push(sum);
          sum = 0;
          continue;
        }

        match line.parse::<i32>() {
          Ok(number) => sum += number,
          Err(error) => panic!("invalid number {}: {}", line, error),
        }
      }

      if sum > 0 {
        calories_per_elf.push(sum)
      }

      calories_per_elf
    }
    Err(error) => panic!("error: {}", error),
  };
}

pub fn run_day_01_part_1() {
  let calories_per_elf = get_calories_per_elf();
  println!("{:?}", calories_per_elf.iter().max().unwrap_or(&0))
}

pub fn run_day_01_part_2() {
  let mut calories_per_elf = get_calories_per_elf();
  calories_per_elf.sort();
  calories_per_elf.reverse();
  println!(
    "{:?}",
    calories_per_elf.get(0..=2).unwrap().iter().sum::<i32>()
  )
}
